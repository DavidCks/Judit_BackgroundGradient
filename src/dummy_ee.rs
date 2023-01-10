use yew::prelude::*;

pub struct Dummy;
pub enum Msg {

}

impl Component for Dummy {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div id="1028c149-e3a7-4314-ab06-178e409d3d2e" jrole="Judit_EditableElement" jdepth="0" inner_html="" jtype="Div" style="position: absolute; box-sizing: border-box; top: 72px; left: 841px; width: 200px; height: 200px; background-color: #EEEEEE; transform-origin: 50%50%; transform-style: flat; transform: skewX( 0deg) skewY( 0deg) translateX( 0px) rotateX( 0deg) rotateY( 0deg) rotateZ( 0deg); border-top: ; border-left: ; border-right: ; border-bottom: ; border-color: #3f3f3f; border-style: solid; border-width: 2px; border-top-left-radius: 10px; border-top-right-radius: 10px; border-bottom-left-radius: 10px; border-bottom-right-radius: 10px; text-align: left; writing-mode: horizontal-tb; text-orientation: upright; text-decoration-line: none; font-weight: normal; font-style: normal; font-size: 1rem; letter-spacing: 0em; word-spacing: 0.25em; line-height: 1; font-family: Arial,sans-serif; opacity: 1; z-index: 1; box-shadow: 0px 0px 5px 1px cornflowerblue; ">
            <div position="TopLeft" jrole="Judit_EditableBorderRadiusSelector" style="position: absolute; width: 11px; height: 11px; top: clamp(0px, 20%, 50px); left: clamp(0px, 20%, 50px); right: ; bottom: ; background-image: none; background-color: #EEEEEE; border-radius: 100%; border-width: 3px; border-color: #3f3f3f; border-style: solid; box-sizing: border-box; z-index: 9999; " class="EditableBorderRadiusSelectorStyle_Hover"></div>
            <div position="TopRight" jrole="Judit_EditableBorderRadiusSelector" style="position: absolute; width: 11px; height: 11px; top: clamp(0px, 20%, 50px); left: ; right: clamp(0px, 20%, 50px); bottom: ; background-image: none; background-color: #EEEEEE; border-radius: 100%; border-width: 3px; border-color: #3f3f3f; border-style: solid; box-sizing: border-box; z-index: 9999; " class="EditableBorderRadiusSelectorStyle_Hover"></div>
            <div position="BottomLeft" jrole="Judit_EditableBorderRadiusSelector" style="position: absolute; width: 11px; height: 11px; top: ; left: clamp(0px, 20%, 50px); right: ; bottom: clamp(0px, 20%, 50px); background-image: none; background-color: #EEEEEE; border-radius: 100%; border-width: 3px; border-color: #3f3f3f; border-style: solid; box-sizing: border-box; z-index: 9999; " class="EditableBorderRadiusSelectorStyle_Hover"></div>
            <div position="BottomRight" jrole="Judit_EditableBorderRadiusSelector" style="position: absolute; width: 11px; height: 11px; top: ; left: ; right: clamp(0px, 20%, 50px); bottom: clamp(0px, 20%, 50px); background-image: none; background-color: #EEEEEE; border-radius: 100%; border-width: 3px; border-color: #3f3f3f; border-style: solid; box-sizing: border-box; z-index: 9999; " class="EditableBorderRadiusSelectorStyle_Hover"></div>
            <svg style="position: absolute; width: 30px; height: 30px; right: -5px; top: calc(-30px - 2px); z-index: 9999;  transform: rotateZ(-0deg) rotateY(-0deg) rotateX(-0deg)" jrole="Judit_DeleteButton" width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                <path jrole="Judit_DeleteButton" d="M9.879 14.121L12 12m2.121-2.121L12 12m0 0L9.879 9.879M12 12l2.121 2.121M21 3.6v16.8a.6.6 0 01-.6.6H3.6a.6.6 0 01-.6-.6V3.6a.6.6 0 01.6-.6h16.8a.6.6 0 01.6.6z" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
            </svg>
            <div jrole="Judit_EditControls" style="position: absolute; width: auto; height: 30px; top: calc(100% + 2px); left: 0px; display: flex; align-items: flex-start; justify-content: left; writing-mode: horizontal-tb; z-index: 9999;  transform: rotateZ(-0deg) rotateY(-0deg) rotateX(-0deg)">
                <svg jrole="Judit_Transform3DToggle" width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000" class="Transform3DToggleStyle Transform3DToggleStyle_hover">
                    <path jrole="Judit_Transform3DToggle" d="M21 7.353v9.294a.6.6 0 01-.309.525l-8.4 4.666a.6.6 0 01-.582 0l-8.4-4.666A.6.6 0 013 16.647V7.353a.6.6 0 01.309-.524l8.4-4.667a.6.6 0 01.582 0l8.4 4.667a.6.6 0 01.309.524z" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
                    <path jrole="Judit_Transform3DToggle" d="M3.528 7.294l8.18 4.544a.6.6 0 00.583 0l8.209-4.56M12 21v-9" stroke="#3f3f3f" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
                </svg>
                <svg jrole="Judit_TextPanelToggle" width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000" class="TextPanelToggleStyle TextPanelToggleStyle_hover">
                    <path jrole="Judit_TextPanelToggle" stroke="#3F3F3F" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M21 3.6v16.8a.6.6 0 0 1-.6.6H3.6a.6.6 0 0 1-.6-.6V3.6a.6.6 0 0 1 .6-.6h16.8a.6.6 0 0 1 .6.6Z"></path>
                    <path jrole="Judit_TextPanelToggle" stroke="#3F3F3F" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M7 9V7h10v2m-5-2v10m0 0h-2m2 0h2"></path>
                </svg>
                <svg jrole="Judit_BorderPanelToggle" width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000" class="BorderPanelToggleStyle BorderPanelToggleStyle_hover">
                    <path jrole="Judit_BorderPanelToggle" stroke="#3F3F3F" stroke-dasharray="2 2" stroke-linecap="round" stroke-linejoin="round" stroke-miterlimit="1.5" stroke-width="1.499" d="M16 2H8a6 6 0 0 0-6 6v8a6 6 0 0 0 6 6h8a6 6 0 0 0 6-6V8a6 6 0 0 0-6-6Z"></path>
                    <path jrole="Judit_BorderPanelToggle" stroke="#3F3F3F" stroke-linecap="round" stroke-linejoin="round" stroke-miterlimit="1.5" stroke-width="1.499" d="M16 5H8a3 3 0 0 0-3 3v8a3 3 0 0 0 3 3h8a3 3 0 0 0 3-3V8a3 3 0 0 0-3-3Z"></path>
                </svg>
            </div>
            <div style="position: absolute; left: calc(100% + 4px); top: 0px; width: fit-content; writing-mode: horizontal-tb; direction: ltr; z-index: 9999;  transform: rotateZ(-0deg) rotateY(-0deg) rotateX(-0deg)">
                <div style="display: grid; grid-template-columns: 1fr 1fr 1fr 1fr; gap: 0px 0px; align-items: center; position: initial; border-radius: 10px; ">
                    <svg style="position: initial; " jrole="Judit_AlignRight" width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path d="M7 10h14M3 6h18M7 18h14M3 14h18" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
                    </svg>
                    <svg style="position: initial; " width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path d="M3 6h18M3 14h18M6 10h12M6 18h12" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
                    </svg>
                    <svg style="position: initial; " width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path d="M3 6h18M3 10h18M3 14h18M3 18h18" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
                    </svg>
                    <svg style="position: initial; " width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path d="M3 10h14M3 6h18M3 18h14M3 14h18" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
                    </svg>
                    <svg style="position: initial; " width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path d="M12 11.667H8m4 0s3.333 0 3.333-3.334C15.333 5 12 5 12 5s0 0 0 0H8.6a.6.6 0 00-.6.6v6.067m4 0s4 0 4 3.666C16 19 12 19 12 19s0 0 0 0H8.6a.6.6 0 01-.6-.6v-6.733" stroke="#000000" stroke-width="1.5"></path>
                    </svg>
                    <svg style="position: initial; " width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path d="M11 5h3m3 0h-3m0 0l-4 14m0 0H7m3 0h3" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
                    </svg>
                    <svg style="position: initial; " width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path d="M16 5v6a4 4 0 01-4 4v0a4 4 0 01-4-4V5M6 19h12" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
                    </svg>
                    <svg jrole="Judit_StyleSizeButton" style="position: initial; " width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path d="M18 21V11m0 10l-2-2.5m2 2.5l2-2.5M18 11l-2 2m2-2l2 2M9 5v12m0 0H7m2 0h2M15 7V5H3v2" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
                    </svg>
                    <svg jrole="Judit_SpacingLinesButton" style="position: initial; " width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path d="M11 7h9M11 12h9M11 17h9M6 17V7m0 10l-2-2.5M6 17l2-2.5M6 7L4 9m2-2l2 2" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
                    </svg>
                    <svg jrole="Judit_SpacingWordsButton" style="position: initial; " width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path d="M4 7h2.824m3.764.04h2.824M17 7h2.823m-.058 7.823H4.235m15.53 0-2.118 2.118m2.118-2.117-2.118-2.118M4.235 14.823l2.118 2.118m-2.118-2.117 2.118-2.118" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
                    </svg>
                    <svg jrole="Judit_SpacingLettersButton" style="position: initial; " width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path d="M4.235 7v10m15.53-10v10m-2.824-5H7.06m9.882 0-2.117 2.118M16.94 12l-2.117-2.118M7.058 12l2.117 2.118M7.06 12l2.117-2.118" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
                    </svg>
                    <svg style="position: initial; " width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path d="M11.122 6.122v8m-5-8v4m10 7.878V6m0 12-2-2.118m2 2.118 2-2.118" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path>
                    </svg>
                    <select class="FontPickerStyle">
                        <option style="font-family: Arial, sans-serif" value="Arial, sans-serif">{"Arial (sans-serif)"}</option>
                        <option style="Verdana, sans-serif" value="Verdana, sans-serif">{"Verdana (sans-serif)"}</option>
                        <option style="Tahoma, sans-serif" value="Tahoma, sans-serif">{"Tahoma (sans-serif)"}</option>
                        <option style="Trebuchet MS, sans-serif" value="Trebuchet MS, sans-serif">{"Trebuchet MS (sans-serif)"}</option>
                        <option style="Times New Roman, serif" value="Times New Roman, serif">{"Times New Roman (serif)"}</option>
                        <option style="Georgia, serif" value="Georgia, serif">{"Georgia (serif)"}</option>
                        <option style="Garamond, serif" value="Garamond, serif">{"Garamond (serif)"}</option>
                        <option style="Courier New, monospace" value="Courier New, monospace">{"Courier New (monospace)"}</option>
                        <option style="Brush Script MT, cursive" value="Brush Script MT, cursive">{"Brush Script MT (cursive)"}</option>
                    </select>
                </div>
                <div style="display: grid; grid-template-columns: 1fr 1fr 1fr 1fr; gap: 0px 0px; align-items: center; position: initial; border-radius: 10px; ">
                    <svg style="position: initial; " width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path stroke="#3F3F3F" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="m8 20.01.01-.011m3.99.011.01-.011m3.99.011.01-.011m3.99.011.01-.011M20 16.01l.01-.011M20 12.01l.01-.011M20 8.01l.01-.011M4 20V4h16"></path>
                    </svg>
                    <svg style="position: initial; " width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path stroke="#3F3F3F" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="m20.986 20.08.01-.012m-4.024.012.01-.012m-4.024.012.01-.012m-4.024.012.01-.012m-4.024.012.01-.012M4.93 8.037l.01-.01m-.01 4.024.01-.01m8.018.01.01-.01M4.93 16.065l.01-.01m16.046-8.018.01-.01m-.01 4.024.01-.01m-.01 4.024.01-.01M4.93 4.014h16.056"></path>
                    </svg>
                    <svg style="position: initial; " width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path stroke="#3F3F3F" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M20.01 20V4h-16M4 8l.012-.01M4 12l.012-.01M4 16l.012-.01M4 20l.012-.01M16 20l.012-.01M12 20l.012-.01M8 20l.012-.01"></path>
                    </svg>
                    <svg jrole="Judit_BorderResizeButton" style="position: initial; " width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path jrole="Judit_BorderResizeButton" stroke="#3f3f3f" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M7 9.5 9.5 12 7 14.5m9.5-5L14 12l2.5 2.5"></path>
                        <path jrole="Judit_BorderResizeButton" stroke="#3f3f3f" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M6 5h12a4 4 0 0 1 4 4v6a4 4 0 0 1-4 4H6a4 4 0 0 1-4-4V9a4 4 0 0 1 4-4Z"></path>
                    </svg>
                    <svg style="position: initial; " width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path stroke="#3F3F3F" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="m20.011 20-.012.01m.012-4.01-.012.01m.012-4.01-.012.01M20.011 8 20 8.01M20.011 4 20 4.01M8.011 4 8 4.01M12.011 4 12 4.01m.011 7.99-.012.01M16.011 4 16 4.01M8.011 20 8 20.01m4.011-.01-.012.01m4.012-.01-.012.01M4 4v16"></path>
                    </svg>
                    <svg style="position: initial; " width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path stroke="#3F3F3F" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="m12.01 16-.01.01m.01-4.01-.01.01M12.01 8l-.01.01M8.01 12l-.01.01m8.01-.01-.01.01m5-8.41v16.8a.6.6 0 0 1-.6.6H3.6a.6.6 0 0 1-.6-.6V3.6a.6.6 0 0 1 .6-.6h16.8a.6.6 0 0 1 .6.6Z"></path>
                    </svg>
                    <svg style="position: initial; " width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path stroke="#3F3F3F" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="m4 20 .012.01M4 16l.012.01M4 12l.012.01M4 8l.012.01M4 4l.012.01M16 4l.012.01M12 4l.012.01M12 12l.012.01M8 4l.012.01M16 20l.012.01M12 20l.012.01M8 20l.012.01M20.01 4v16"></path>
                    </svg>
                    <svg style="position: initial; " width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path stroke="#3f3f3f" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="m3 3 18 18M10.5 10.677a2 2 0 0 0 2.823 2.823"></path>
                        <path stroke="#3f3f3f" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M7.362 7.561C5.68 8.74 4.279 10.42 3 12c1.889 2.991 5.282 6 9 6 1.55 0 3.043-.523 4.395-1.35M12 6c4.008 0 6.701 3.158 9 6a15.66 15.66 0 0 1-1.078 1.5"></path>
                    </svg>
                    <svg style="position: initial; " width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path stroke="#3F3F3F" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M4 4v16h16m.011-4-.012.01m.012-4.01-.012.01M20.011 8 20 8.01M20.011 4 20 4.01M8.011 4 8 4.01M12.011 4 12 4.01M16.011 4 16 4.01"></path>
                    </svg>
                    <svg style="position: initial; " width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path stroke="#3F3F3F" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="m20 4.01.01-.011M16 4.01l.01-.011M12 4.01l.01-.011M8 4.01l.01-.011M4 4.01l.01-.011M4 8.01l.01-.011M4 12.01l.01-.011m7.99.011.01-.011M4 16.01l.01-.011M20 8.01l.01-.011M20 12.01l.01-.011M20 16.01l.01-.011M4 20h16"></path>
                    </svg>
                    <svg style="position: initial; " width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                        <path stroke="#3F3F3F" stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M20.01 4v16h-16M4 16l.012.01M4 12l.012.01M4 8l.012.01M4 4l.012.01M16 4l.012.01M12 4l.012.01M8 4l.012.01"></path>
                    </svg>
                    <div style="position: initial; background-color: #3f3f3f; width: 18px; height: 18px; border: none; border-radius: 5px; appearance: none; padding: 0px; margin: 3px; "><input style="opacity: 0; width: 100%; height: 100%" type="color"/></div>
                </div>
            </div>
            </div>

        }
    }
}