use yew::virtual_dom::{VNode, VRaw};
use yew::{AttrValue, Html};

pub const CORRECT_SVG: Html = VNode::VRaw(VRaw {
    html: AttrValue::Static(include_str!("../../assets/correct.svg")),
});
pub const FAST_FORWARD_SVG: Html = VNode::VRaw(VRaw {
    html: AttrValue::Static(include_str!("../../assets/fast-forward.svg")),
});
pub const INCORRECT_SVG: Html = VNode::VRaw(VRaw {
    html: AttrValue::Static(include_str!("../../assets/incorrect.svg")),
});
pub const PENCIL_SVG: Html = VNode::VRaw(VRaw {
    html: AttrValue::Static(include_str!("../../assets/pencil.svg")),
});
