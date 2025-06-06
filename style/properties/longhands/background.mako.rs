/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

<%namespace name="helpers" file="/helpers.mako.rs" />

${helpers.predefined_type(
    "background-color",
    "Color",
    "computed::Color::TRANSPARENT_BLACK",
    engines="gecko servo",
    initial_specified_value="SpecifiedValue::transparent()",
    spec="https://drafts.csswg.org/css-backgrounds/#background-color",
    ignored_when_colors_disabled=True,
    allow_quirks="Yes",
    flags="CAN_ANIMATE_ON_COMPOSITOR",
    servo_restyle_damage="repaint",
    affects="paint",
)}

${helpers.predefined_type(
    "background-image",
    "Image",
    engines="gecko servo",
    initial_value="computed::Image::None",
    initial_specified_value="specified::Image::None",
    spec="https://drafts.csswg.org/css-backgrounds/#the-background-image",
    vector="True",
    animation_type="discrete",
    ignored_when_colors_disabled="True",
    servo_restyle_damage="repaint",
    affects="paint",
)}

% for (axis, direction, initial) in [("x", "Horizontal", "left"), ("y", "Vertical", "top")]:
    ${helpers.predefined_type(
        "background-position-" + axis,
        "position::" + direction + "Position",
        "computed::LengthPercentage::zero_percent()",
        engines="gecko servo",
        initial_specified_value="SpecifiedValue::initial_specified_value()",
        spec="https://drafts.csswg.org/css-backgrounds-4/#propdef-background-position-" + axis,
        vector=True,
        vector_animation_type="repeatable_list",
        servo_restyle_damage="repaint",
        affects="paint",
    )}
% endfor

${helpers.predefined_type(
    "background-repeat",
    "BackgroundRepeat",
    "computed::BackgroundRepeat::repeat()",
    engines="gecko servo",
    initial_specified_value="specified::BackgroundRepeat::repeat()",
    animation_type="discrete",
    vector=True,
    spec="https://drafts.csswg.org/css-backgrounds/#the-background-repeat",
    servo_restyle_damage="repaint",
    affects="paint",
)}

${helpers.single_keyword(
    "background-attachment",
    "scroll fixed" + (" local" if engine == "gecko" else ""),
    engines="gecko servo",
    vector=True,
    gecko_enum_prefix="StyleImageLayerAttachment",
    spec="https://drafts.csswg.org/css-backgrounds/#the-background-attachment",
    animation_type="discrete",
    servo_restyle_damage="repaint",
    affects="paint",
)}

${helpers.single_keyword(
    "background-clip",
    "border-box padding-box content-box",
    engines="gecko servo",
    extra_gecko_values="text",
    vector=True, extra_prefixes="webkit",
    gecko_enum_prefix="StyleGeometryBox",
    gecko_inexhaustive=True,
    spec="https://drafts.csswg.org/css-backgrounds/#the-background-clip",
    animation_type="discrete",
    servo_restyle_damage="repaint",
    affects="paint",
)}

${helpers.single_keyword(
    "background-origin",
    "padding-box border-box content-box",
    engines="gecko servo",
    vector=True, extra_prefixes="webkit",
    gecko_enum_prefix="StyleGeometryBox",
    gecko_inexhaustive=True,
    spec="https://drafts.csswg.org/css-backgrounds/#the-background-origin",
    animation_type="discrete",
    servo_restyle_damage="repaint",
    affects="paint",
)}

${helpers.predefined_type(
    "background-size",
    "BackgroundSize",
    engines="gecko servo",
    initial_value="computed::BackgroundSize::auto()",
    initial_specified_value="specified::BackgroundSize::auto()",
    spec="https://drafts.csswg.org/css-backgrounds/#the-background-size",
    vector=True,
    vector_animation_type="repeatable_list",
    extra_prefixes="webkit",
    servo_restyle_damage="repaint",
    affects="paint",
)}

// https://drafts.fxtf.org/compositing/#background-blend-mode
${helpers.single_keyword(
    "background-blend-mode",
    """normal multiply screen overlay darken lighten color-dodge
    color-burn hard-light soft-light difference exclusion hue
    saturation color luminosity""",
    gecko_enum_prefix="StyleBlend",
    vector=True,
    engines="gecko",
    animation_type="discrete",
    gecko_inexhaustive=True,
    spec="https://drafts.fxtf.org/compositing/#background-blend-mode",
    affects="paint",
)}
