//! `flash.display.Stage` builtin/prototype

use crate::avm2::activation::Activation;
use crate::avm2::error::argument_error;
use crate::avm2::object::{Object, TObject};
use crate::avm2::parameters::ParametersExt;
use crate::avm2::value::Value;
use crate::avm2::Error;
use crate::avm2::{ArrayObject, ArrayStorage};
use crate::display_object::{StageDisplayState, TDisplayObject};
use crate::string::{AvmString, WString};
use crate::{avm2_stub_getter, avm2_stub_setter};
use swf::Color;

/// Implements `flash.display.Stage`'s native instance constructor.
pub fn native_instance_init<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this {
        activation.super_init(this, args)?;
    }

    Ok(Value::Undefined)
}

/// Implement `align`'s getter
pub fn get_align<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let align = activation.context.stage.align();
    let mut s = WString::with_capacity(4, false);
    // Match string values returned by AS.
    // It's possible to have an oxymoronic "TBLR".
    // This acts the same as "TL" (top-left takes priority).
    // This order is different between AVM1 and AVM2!
    use crate::display_object::StageAlign;
    if align.contains(StageAlign::TOP) {
        s.push_byte(b'T');
    }
    if align.contains(StageAlign::BOTTOM) {
        s.push_byte(b'B');
    }
    if align.contains(StageAlign::LEFT) {
        s.push_byte(b'L');
    }
    if align.contains(StageAlign::RIGHT) {
        s.push_byte(b'R');
    }
    let align = AvmString::new(activation.context.gc_context, s);
    Ok(align.into())
}

/// Implement `align`'s setter
pub fn set_align<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let align = args.get_string(activation, 0)?.parse().unwrap_or_default();
    activation
        .context
        .stage
        .set_align(&mut activation.context, align);
    Ok(Value::Undefined)
}

/// Implement `browserZoomFactor`'s getter
pub fn get_browser_zoom_factor<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if this
        .and_then(|this| this.as_display_object())
        .and_then(|this| this.as_stage())
        .is_some()
    {
        return Ok(activation
            .context
            .renderer
            .viewport_dimensions()
            .scale_factor
            .into());
    }

    Ok(Value::Undefined)
}

/// Implement `color`'s getter
pub fn get_color<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(dobj) = this
        .and_then(|this| this.as_display_object())
        .and_then(|this| this.as_stage())
    {
        let color = dobj.background_color().unwrap_or(Color::WHITE);
        return Ok(color.to_rgba().into());
    }

    Ok(Value::Undefined)
}

/// Implement `color`'s setter
pub fn set_color<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(dobj) = this
        .and_then(|this| this.as_display_object())
        .and_then(|this| this.as_stage())
    {
        let color = Color::from_rgb(args.get_u32(activation, 0)?, 255);
        dobj.set_background_color(activation.context.gc_context, Some(color));
    }

    Ok(Value::Undefined)
}

/// Implement `contentsScaleFactor`'s getter
pub fn get_contents_scale_factor<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if this
        .and_then(|this| this.as_display_object())
        .and_then(|this| this.as_stage())
        .is_some()
    {
        return Ok(activation
            .context
            .renderer
            .viewport_dimensions()
            .scale_factor
            .into());
    }

    Ok(Value::Undefined)
}

/// Implement `displayState`'s getter
pub fn get_display_state<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let display_state = AvmString::new_utf8(
        activation.context.gc_context,
        activation.context.stage.display_state().to_string(),
    );
    Ok(display_state.into())
}

/// Implement `displayState`'s setter
pub fn set_display_state<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Ok(mut display_state) = args.get_string(activation, 0)?.parse() {
        // It's not entirely clear why when setting to FullScreen, desktop flash player at least will
        // set its value to FullScreenInteractive. Overriding until flash logic is clearer.
        if display_state == StageDisplayState::FullScreen {
            display_state = StageDisplayState::FullScreenInteractive;
        }
        activation
            .context
            .stage
            .set_display_state(&mut activation.context, display_state);
    } else {
        return Err(Error::AvmError(argument_error(
            activation,
            "Error #2008: Parameter displayState must be one of the accepted values.",
            2008,
        )?));
    }
    Ok(Value::Undefined)
}

/// Implement `focus`'s getter
pub fn get_focus<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    Ok(activation
        .context
        .focus_tracker
        .get()
        .and_then(|focus_dobj| focus_dobj.object2().as_object())
        .map(|o| o.into())
        .unwrap_or(Value::Null))
}

/// Implement `focus`'s setter
pub fn set_focus<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let focus = activation.context.focus_tracker;
    match args.try_get_object(activation, 0) {
        None => focus.set(None, &mut activation.context),
        Some(obj) => {
            if let Some(dobj) = obj.as_display_object() {
                focus.set(Some(dobj), &mut activation.context);
            } else {
                return Err("Cannot set focus to non-DisplayObject".into());
            }
        }
    };

    Ok(Value::Undefined)
}

/// Implement `frameRate`'s getter
pub fn get_frame_rate<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    Ok((*activation.context.frame_rate).into())
}

/// Implement `frameRate`'s setter
pub fn set_frame_rate<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let new_frame_rate = args.get_f64(activation, 0)?;
    *activation.context.frame_rate = new_frame_rate;

    Ok(Value::Undefined)
}

pub fn get_show_default_context_menu<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    Ok(activation.context.stage.show_menu().into())
}

pub fn set_show_default_context_menu<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let show_default_context_menu = args.get_bool(0);
    activation
        .context
        .stage
        .set_show_menu(&mut activation.context, show_default_context_menu);
    Ok(Value::Undefined)
}

/// Implement `scaleMode`'s getter
pub fn get_scale_mode<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let scale_mode = AvmString::new_utf8(
        activation.context.gc_context,
        activation.context.stage.scale_mode().to_string(),
    );
    Ok(scale_mode.into())
}

/// Implement `scaleMode`'s setter
pub fn set_scale_mode<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Ok(scale_mode) = args.get_string(activation, 0)?.parse() {
        activation
            .context
            .stage
            .set_scale_mode(&mut activation.context, scale_mode);
    } else {
        return Err(Error::AvmError(argument_error(
            activation,
            "Error #2008: Parameter scaleMode must be one of the accepted values.",
            2008,
        )?));
    }
    Ok(Value::Undefined)
}

/// Implement `stageFocusRect`'s getter
///
/// This setting is currently ignored in Ruffle.
pub fn get_stage_focus_rect<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(dobj) = this
        .and_then(|this| this.as_display_object())
        .and_then(|this| this.as_stage())
    {
        return Ok(dobj.stage_focus_rect().into());
    }

    Ok(Value::Undefined)
}

/// Implement `stageFocusRect`'s setter
///
/// This setting is currently ignored in Ruffle.
pub fn set_stage_focus_rect<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(dobj) = this
        .and_then(|this| this.as_display_object())
        .and_then(|this| this.as_stage())
    {
        let rf = args.get_bool(0);
        dobj.set_stage_focus_rect(activation.context.gc_context, rf);
    }

    Ok(Value::Undefined)
}

/// Implement `stageWidth`'s getter
pub fn get_stage_width<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(dobj) = this
        .and_then(|this| this.as_display_object())
        .and_then(|this| this.as_stage())
    {
        return Ok(dobj.stage_size().0.into());
    }

    Ok(Value::Undefined)
}

/// Implement `stageWidth`'s setter
pub fn set_stage_width<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    // For some reason this value is settable but it does nothing.
    Ok(Value::Undefined)
}

/// Implement `stageHeight`'s getter
pub fn get_stage_height<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(dobj) = this
        .and_then(|this| this.as_display_object())
        .and_then(|this| this.as_stage())
    {
        return Ok(dobj.stage_size().1.into());
    }

    Ok(Value::Undefined)
}

/// Implement `stageHeight`'s setter
pub fn set_stage_height<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    // For some reason this value is settable but it does nothing.
    Ok(Value::Undefined)
}

/// Implement `allowsFullScreen`'s getter
pub fn get_allows_full_screen<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_getter!(activation, "flash.display.Stage", "allowsFullScreen");
    Ok(true.into())
}

/// Implement `allowsFullScreenInteractive`'s getter
pub fn get_allows_full_screen_interactive<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_getter!(
        activation,
        "flash.display.Stage",
        "allowsFullScreenInteractive"
    );
    Ok(false.into())
}

/// Implement `quality`'s getter
pub fn get_quality<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let quality = activation.context.stage.quality().into_avm_str();
    Ok(AvmString::from(quality).into())
}

/// Implement `quality`'s setter
pub fn set_quality<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    // Invalid values result in no change.
    if let Ok(quality) = args.get_string(activation, 0)?.parse() {
        activation
            .context
            .stage
            .set_quality(&mut activation.context, quality);
    }
    Ok(Value::Undefined)
}

/// Implement `stage3Ds`'s getter
pub fn get_stage3ds<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(stage) = this
        .and_then(|this| this.as_display_object())
        .and_then(|this| this.as_stage())
    {
        let storage = ArrayStorage::from_storage(
            stage
                .stage3ds()
                .iter()
                .map(|obj| Some(Value::Object(*obj)))
                .collect(),
        );
        let stage3ds_array = ArrayObject::from_storage(activation, storage)?;
        return Ok(stage3ds_array.into());
    }
    Ok(Value::Undefined)
}

/// Implement `invalidate`
pub fn invalidate<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(stage) = this
        .and_then(|this| this.as_display_object())
        .and_then(|this| this.as_stage())
    {
        stage.set_invalidated(activation.context.gc_context, true);
    }
    Ok(Value::Undefined)
}

/// Stage.fullScreenSourceRect's getter
pub fn get_full_screen_source_rect<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_getter!(activation, "flash.display.Stage", "fullScreenSourceRect");
    Ok(Value::Undefined)
}

/// Stage.fullScreenSourceRect's setter
pub fn set_full_screen_source_rect<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_setter!(activation, "flash.display.Stage", "fullScreenSourceRect");
    Ok(Value::Undefined)
}

/// Stage.fullScreenHeight's getter
pub fn get_full_screen_height<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_getter!(activation, "flash.display.Stage", "fullScreenHeight");
    Ok(768.into())
}

/// Stage.fullScreenWidth's getter
pub fn get_full_screen_width<'gc>(
    activation: &mut Activation<'_, 'gc>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_getter!(activation, "flash.display.Stage", "fullScreenWidth");
    Ok(1024.into())
}
