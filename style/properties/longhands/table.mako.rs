/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

<%namespace name="helpers" file="/helpers.mako.rs" />

${helpers.single_keyword(
    "table-layout",
    "auto fixed",
    engines="gecko servo",
    gecko_ffi_name="mLayoutStrategy",
    animation_type="discrete",
    gecko_enum_prefix="StyleTableLayout",
    spec="https://drafts.csswg.org/css-tables/#propdef-table-layout",
    servo_restyle_damage="rebuild_box",
    affects="layout",
)}

${helpers.predefined_type(
    "-x-span",
    "Integer",
    "1",
    engines="gecko",
    spec="Internal-only (for `<col span>` pres attr)",
    animation_type="none",
    enabled_in="",
    affects="layout",
)}
