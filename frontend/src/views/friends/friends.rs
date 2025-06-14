use leptos::prelude::*;
use thaw::*;

#[component]
pub fn Friends() -> impl IntoView {
    let on_select = move |key: String| {
        leptos::logging::warn!("{}", key);
    };

    view! {
        <Space>
            <Menu on_select trigger_type=MenuTriggerType::Hover>
                <MenuTrigger slot>
                    <Button>"Hover"</Button>
                </MenuTrigger>
                <MenuItem value="facebook">"Facebook"</MenuItem>
                <MenuItem value="twitter" disabled=true>
                    "Twitter"
                </MenuItem>
            </Menu>

            <Menu on_select>
                <MenuTrigger slot>
                    <Button>"Click"</Button>
                </MenuTrigger>
                <MenuItem value="facebook">"Facebook"</MenuItem>
                <MenuItem value="twitter">"Twitter"</MenuItem>
                <MenuItem value="no_icon" disabled=true>
                    "Mastodon"
                </MenuItem>
            </Menu>
        </Space>
    }
}
