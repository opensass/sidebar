use sidebar::yew::item::MenuItem;
use sidebar::yew::menu::Menu;
use sidebar::yew::sidebar::Sidebar;
use sidebar::yew::submenu::Submenu;
use yew::prelude::*;

#[function_component(Example1)]
pub fn example1() -> Html {
    let selected_label = use_state(|| "Overview".to_string());
    let logout = Callback::from(|_| log::info!("Logging out"));

    html! {
        <Sidebar
            user_name="Ferris"
            designation="Engineer"
            user_img="/assets/logo.webp"
            logo_img_url="/assets/logo.webp"
            on_logout={logout}
            style="width: 300px; background: #1a202c; color: white;"
            class="shadow-lg"
        >
            <Menu sub_heading="MAIN">
                <MenuItem
                    label="Overview"
                    href="#"
                    icon_html={html!{<i class="fas fa-home"></i>}}
                    selected={selected_label.clone()}
                />
                <MenuItem
                    label="Activity"
                    href="#"
                    icon_html={html!{<i class="fas fa-stream"></i>}}
                    selected={selected_label.clone()}
                />
            </Menu>
        </Sidebar>
    }
}

#[function_component(Example2)]
pub fn example2() -> Html {
    let selected_label = use_state(|| "".to_string());
    html! {
        <Sidebar
            show_profile=false
            logo_img_url="/assets/logo.webp"
            style="background: #f7fafc; border-right: 1px solid #e2e8f0;"
        >
            <Menu sub_heading="Minimal Sidebar">
                <MenuItem
                    label="Home"
                    href="#"
                    icon_html={html!{<i class="fas fa-house-user"></i>}}
                    selected={selected_label.clone()}
                />
                <MenuItem
                    label="Settings"
                    href="#"
                    icon_html={html!{<i class="fas fa-tools"></i>}}
                    selected={selected_label.clone()}
                />
            </Menu>
        </Sidebar>
    }
}

#[function_component(Example3)]
pub fn example3() -> Html {
    let logout_callback = Callback::from(|_| log::info!("Logout clicked"));
    let selected_label = use_state(|| "".to_string());

    html! {
        <Sidebar
            user_name="Ferris"
            designation="Product Manager"
            user_img="/assets/logo.webp"
            logo_img_url="/assets/logo.webp"
            on_logout={logout_callback}
            class="w-64 bg-black text-gray-800 h-screen border-r border-gray-200"
        >
            <Menu sub_heading="DASHBOARD">
                <MenuItem
                    label="Overview"
                    href="#"
                    icon_html={html! {<i class="fas fa-chart-pie"></i>}}
                    badge={Some("New")}
                    item_class="flex items-center justify-between px-4 py-2 rounded-md"
                    selected={selected_label.clone()}
                />
                <MenuItem
                    label="Reports"
                    href="#"
                    icon_html={html! {<i class="fas fa-file-alt"></i>}}
                    item_class="flex items-center px-4 py-2 rounded-md hover:bg-gray-100 transition"
                    selected={selected_label.clone()}
                />
                <MenuItem
                    label="Performance"
                    href="#"
                    icon_html={html! {<i class="fas fa-tachometer-alt"></i>}}
                    item_class="flex items-center px-4 py-2 rounded-md hover:bg-gray-100 transition"
                    selected={selected_label.clone()}
                />
            </Menu>
            <Menu sub_heading="MANAGEMENT">
                <MenuItem
                    label="Users"
                    href="#"
                    icon_html={html! {<i class="fas fa-users"></i>}}
                    item_class="flex items-center px-4 py-2 rounded-md hover:bg-gray-100 transition"
                    selected={selected_label.clone()}
                />
                <MenuItem
                    label="Teams"
                    href="#"
                    icon_html={html! {<i class="fas fa-user-friends"></i>}}
                    item_class="flex items-center px-4 py-2 rounded-md hover:bg-gray-100 transition"
                    selected={selected_label.clone()}
                />
                <Submenu title="Settings" icon_html={html! {<i class="fas fa-cog"></i>}}>
                    <MenuItem
                        label="Profile"
                        href="#"
                        icon_html={html! {<i class="fas fa-id-badge"></i>}}
                        item_class="flex items-center px-4 py-2 rounded-md hover:bg-gray-100 transition"
                        selected={selected_label.clone()}
                    />
                    <MenuItem
                        label="Security"
                        href="#"
                        icon_html={html! {<i class="fas fa-shield-alt"></i>}}
                        item_class="flex items-center px-4 py-2 rounded-md hover:bg-gray-100 transition"
                        selected={selected_label.clone()}
                    />
                </Submenu>
            </Menu>
            <Menu sub_heading="RESOURCES">
                <MenuItem
                    label="Documentation"
                    href="#"
                    icon_html={html! {<i class="fas fa-book"></i>}}
                    badge={Some("v2.3")}
                    item_class="flex items-center justify-between px-4 py-2 rounded-md hover:bg-gray-100 transition"
                    selected={selected_label.clone()}
                />
                <MenuItem
                    label="Support"
                    href="#"
                    icon_html={html! {<i class="fas fa-life-ring"></i>}}
                    item_class="flex items-center px-4 py-2 rounded-md hover:bg-gray-100 transition"
                    selected={selected_label.clone()}
                />
            </Menu>
        </Sidebar>
    }
}

#[function_component(Example4)]
pub fn example4() -> Html {
    let selected_label = use_state(|| "".to_string());
    html! {
        <Sidebar
            user_name="Ferris"
            designation="Team Lead"
            user_img="/assets/logo.webp"
            logo_img_url="/assets/logo.webp"
            header_class="transition-all duration-500 ease-in-out transform hover:scale-105"
        >
            <Menu sub_heading="Hover Effects">
                <MenuItem
                    label="Dashboard"
                    href="#"
                    icon_html={html!{<i class="fas fa-tachometer-alt"></i>}}
                    selected={selected_label.clone()}
                />
                <MenuItem
                    label="Team"
                    href="#"
                    icon_html={html!{<i class="fas fa-users-cog"></i>}}
                    selected={selected_label.clone()}
                />
            </Menu>
        </Sidebar>
    }
}

#[function_component(Example5)]
pub fn example5() -> Html {
    let selected_label = use_state(|| "".to_string());
    html! {
        <Sidebar user_name="Ferris" designation="Analyst" user_img="/assets/logo.webp">
            <Menu sub_heading="ANALYTICS">
                <MenuItem
                    label="Reports"
                    href="#"
                    icon_html={html!{<i class="fas fa-chart-bar"></i>}}
                    selected={selected_label.clone()}
                    badge={Some("Beta")}
                />
                <MenuItem
                    label="Insights"
                    href="#"
                    icon_html={html!{<i class="fas fa-eye"></i>}}
                    selected={selected_label.clone()}
                />
            </Menu>
            <Menu sub_heading="ADMIN">
                <Submenu title="Settings" icon_html={html!{<i class="fas fa-cogs"></i>}}>
                    <MenuItem
                        label="Users"
                        href="#"
                        icon_html={html!{<i class="fas fa-user-shield"></i>}}
                        selected={selected_label.clone()}
                    />
                    <MenuItem
                        label="Roles"
                        href="#"
                        icon_html={html!{<i class="fas fa-user-tag"></i>}}
                        selected={selected_label.clone()}
                    />
                </Submenu>
            </Menu>
        </Sidebar>
    }
}

#[function_component(Example6)]
pub fn example6() -> Html {
    let selected_label = use_state(|| "".to_string());
    html! {
        <Sidebar show_profile=false logo_img_url="/assets/logo.webp">
            <Menu sub_heading="MANAGEMENT">
                <MenuItem
                    label="Employees"
                    href="#"
                    icon_html={html!{<i class="fas fa-briefcase"></i>}}
                    selected={selected_label.clone()}
                />
                <MenuItem
                    label="Payroll"
                    href="#"
                    icon_html={html!{<i class="fas fa-file-invoice-dollar"></i>}}
                    selected={selected_label.clone()}
                />
            </Menu>
        </Sidebar>
    }
}

#[function_component(Example7)]
pub fn example7() -> Html {
    let selected_label = use_state(|| "".to_string());
    html! {
        <Sidebar
            user_name="Ferris"
            designation="DevOps"
            user_img="/assets/logo.webp"
            style="background: #0f172a;"
            class="text-white transition duration-700 ease-in-out transform hover:translate-x-1"
        >
            <Menu sub_heading="OPS">
                <MenuItem
                    label="Pipelines"
                    href="#"
                    icon_html={html!{<i class="fas fa-code-branch"></i>}}
                    selected={selected_label.clone()}
                />
                <MenuItem
                    label="Deployments"
                    href="#"
                    icon_html={html!{<i class="fas fa-server"></i>}}
                    selected={selected_label.clone()}
                />
            </Menu>
        </Sidebar>
    }
}

#[function_component(Example8)]
pub fn example8() -> Html {
    let selected_label = use_state(|| "".to_string());
    html! {
        <Sidebar
            user_name="Ferris"
            designation="Product Owner"
            user_img="/assets/logo.webp"
            logo_img_url="/assets/logo.webp"
        >
            <Menu sub_heading="FEATURE FLAGS">
                <MenuItem
                    label="New UI"
                    href="#"
                    icon_html={html!{<i class="fas fa-toggle-on"></i>}}
                    selected={selected_label.clone()}
                    badge={Some("On")}
                />
                <MenuItem
                    label="Beta Tools"
                    href="#"
                    icon_html={html!{<i class="fas fa-tools"></i>}}
                    selected={selected_label.clone()}
                    badge={Some("Test")}
                />
            </Menu>
        </Sidebar>
    }
}

#[function_component(Example9)]
pub fn example9() -> Html {
    let selected_label = use_state(|| "".to_string());
    html! {
        <Sidebar
            user_name="Ferris"
            designation="Marketing"
            user_img="/assets/logo.webp"
            style="width: 100%; background: linear-gradient(to left, #f9fafb, #d1d5db);"
        >
            <Menu sub_heading="CAMPAIGNS">
                <MenuItem
                    label="Ad Spend"
                    href="#"
                    icon_html={html!{<i class="fas fa-bullhorn"></i>}}
                    selected={selected_label.clone()}
                />
                <MenuItem
                    label="Audience"
                    href="#"
                    icon_html={html!{<i class="fas fa-users"></i>}}
                    selected={selected_label.clone()}
                />
            </Menu>
        </Sidebar>
    }
}

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-8 text-white">{ "Sidebar Yew Examples" }</h1>
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Basic Dashboard" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use sidebar::yew::item::MenuItem;
use sidebar::yew::sidebar::Sidebar;
use sidebar::yew::menu::Menu;

#[function_component(Example1)]
pub fn example1() -> Html {
    let selected_label = use_state(|| "Overview".to_string());
    let logout = Callback::from(|_| log::info!("Logging out"));
    html! {
        <Sidebar user_name="Ferris" designation="Engineer" user_img="/assets/logo.webp" logo_img_url="/assets/logo.webp" on_logout={logout} style="width: 300px; background: #1a202c; color: white;" class="shadow-lg">
            <Menu sub_heading="MAIN">
                <MenuItem label="Overview" href="\#" icon_html={html!{<i class="fas fa-home"></i>}} selected={selected_label.clone()} />
                <MenuItem label="Activity" href="\#" icon_html={html!{<i class="fas fa-stream"></i>}} selected={selected_label.clone()} />
            </Menu>
        </Sidebar>
    }
}"# }
                    </pre>
                    <Example1 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Minimal Sidebar" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use sidebar::yew::item::MenuItem;
use sidebar::yew::sidebar::Sidebar;
use sidebar::yew::menu::Menu;

#[function_component(Example2)]
pub fn example2() -> Html {
    let selected_label = use_state(|| "".to_string());
    html! {
        <Sidebar show_profile={false} logo_img_url="/assets/logo.webp" style="background: #f7fafc; border-right: 1px solid #e2e8f0;">
            <Menu sub_heading="Minimal Sidebar">
                <MenuItem label="Home" href="\#" icon_html={html!{<i class="fas fa-house-user"></i>}} selected={selected_label.clone()} />
                <MenuItem label="Settings" href="\#" icon_html={html!{<i class="fas fa-tools"></i>}} selected={selected_label.clone()} />
            </Menu>
        </Sidebar>
    }
}"# }
                    </pre>
                    <Example2 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Creative Sidebar" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use sidebar::yew::item::MenuItem;
use sidebar::yew::submenu::Submenu;
use sidebar::yew::sidebar::Sidebar;
use sidebar::yew::menu::Menu;


#[function_component(Example3)]
pub fn example3() -> Html {
    let logout_callback = Callback::from(|_| log::info!("Logout clicked"));
    let selected_label = use_state(|| "".to_string());
    
    html! {
        <Sidebar
            user_name="Ferris"
            designation="Product Manager"
            user_img="/assets/logo.webp"
            logo_img_url="/assets/logo.webp"
            on_logout={logout_callback}
            class="w-64 bg-black text-gray-800 h-screen border-r border-gray-200"
        >
            <Menu sub_heading="DASHBOARD">
                <MenuItem
                    label="Overview"
                    href="\#"
                    icon_html={html! {<i class="fas fa-chart-pie"></i>}}
                    badge={Some("New")}
                    item_class="flex items-center justify-between px-4 py-2 rounded-md"
                    selected={selected_label.clone()}
                />
                <MenuItem
                    label="Reports"
                    href="\#"
                    icon_html={html! {<i class="fas fa-file-alt"></i>}}
                    item_class="flex items-center px-4 py-2 rounded-md hover:bg-gray-100 transition"
                    selected={selected_label.clone()}
                />
                <MenuItem
                    label="Performance"
                    href="\#"
                    icon_html={html! {<i class="fas fa-tachometer-alt"></i>}}
                    item_class="flex items-center px-4 py-2 rounded-md hover:bg-gray-100 transition"
                    selected={selected_label.clone()}
                />
            </Menu>

            <Menu sub_heading="MANAGEMENT">
                <MenuItem
                    label="Users"
                    href="\#"
                    icon_html={html! {<i class="fas fa-users"></i>}}
                    item_class="flex items-center px-4 py-2 rounded-md hover:bg-gray-100 transition"
                    selected={selected_label.clone()}
                />
                <MenuItem
                    label="Teams"
                    href="\#"
                    icon_html={html! {<i class="fas fa-user-friends"></i>}}
                    item_class="flex items-center px-4 py-2 rounded-md hover:bg-gray-100 transition"
                    selected={selected_label.clone()}
                />
                <Submenu title="Settings" icon_html={html! {<i class="fas fa-cog"></i>}}>
                    <MenuItem
                        label="Profile"
                        href="\#"
                        icon_html={html! {<i class="fas fa-id-badge"></i>}}
                        item_class="flex items-center px-4 py-2 rounded-md hover:bg-gray-100 transition"
                        selected={selected_label.clone()}
                    />
                    <MenuItem
                        label="Security"
                        href="\#"
                        icon_html={html! {<i class="fas fa-shield-alt"></i>}}
                        item_class="flex items-center px-4 py-2 rounded-md hover:bg-gray-100 transition"
                        selected={selected_label.clone()}
                    />
                </Submenu>
            </Menu>

            <Menu sub_heading="RESOURCES">
                <MenuItem
                    label="Documentation"
                    href="\#"
                    icon_html={html! {<i class="fas fa-book"></i>}}
                    badge={Some("v2.3")}
                    item_class="flex items-center justify-between px-4 py-2 rounded-md hover:bg-gray-100 transition"
                    selected={selected_label.clone()}
                />
                <MenuItem
                    label="Support"
                    href="\#"
                    icon_html={html! {<i class="fas fa-life-ring"></i>}}
                    item_class="flex items-center px-4 py-2 rounded-md hover:bg-gray-100 transition"
                    selected={selected_label.clone()}
                />
            </Menu>
        </Sidebar>
    }
}"# }
                    </pre>
                    <Example3 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Hover Effects" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use sidebar::yew::item::MenuItem;
use sidebar::yew::sidebar::Sidebar;
use sidebar::yew::menu::Menu;


#[function_component(Example4)]
pub fn example4() -> Html {
    let selected_label = use_state(|| "".to_string());
    html! {
        <Sidebar
            user_name="Ferris"
            designation="Team Lead"
            user_img="/assets/logo.webp"
            logo_img_url="/assets/logo.webp"
            header_class="transition-all duration-500 ease-in-out transform hover:scale-105"
        >
            <Menu sub_heading="Hover Effects">
                <MenuItem label="Dashboard" href="\#" icon_html={html!{<i class="fas fa-tachometer-alt"></i>}} selected={selected_label.clone()} />
                <MenuItem label="Team" href="\#" icon_html={html!{<i class="fas fa-users-cog"></i>}} selected={selected_label.clone()} />
            </Menu>
        </Sidebar>
    }
}"# }
                    </pre>
                    <Example4 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Analytics + Admin" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use sidebar::yew::submenu::Submenu;
use sidebar::yew::item::MenuItem;
use sidebar::yew::sidebar::Sidebar;
use sidebar::yew::menu::Menu;

#[function_component(Example5)]
pub fn example5() -> Html {
    let selected_label = use_state(|| "".to_string());
    html! {
        <Sidebar user_name="Ferris" designation="Analyst" user_img="/assets/logo.webp">
            <Menu sub_heading="ANALYTICS">
                <MenuItem label="Reports" href="\#" icon_html={html!{<i class="fas fa-chart-bar"></i>}} selected={selected_label.clone()} badge={Some("Beta")} />
                <MenuItem label="Insights" href="\#" icon_html={html!{<i class="fas fa-eye"></i>}} selected={selected_label.clone()} />
            </Menu>
            <Menu sub_heading="ADMIN">
                <Submenu title="Settings" icon_html={html!{<i class="fas fa-cogs"></i>}}>
                    <MenuItem label="Users" href="\#" icon_html={html!{<i class="fas fa-user-shield"></i>}} selected={selected_label.clone()} />
                    <MenuItem label="Roles" href="\#" icon_html={html!{<i class="fas fa-user-tag"></i>}} selected={selected_label.clone()} />
                </Submenu>
            </Menu>
        </Sidebar>
    }
}"# }
                    </pre>
                    <Example5 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "HR Management" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use sidebar::yew::item::MenuItem;
use sidebar::yew::sidebar::Sidebar;
use sidebar::yew::menu::Menu;


#[function_component(Example6)]
pub fn example6() -> Html {
    let selected_label = use_state(|| "".to_string());
    html! {
        <Sidebar show_profile={false} logo_img_url="/assets/logo.webp">
            <Menu sub_heading="MANAGEMENT">
                <MenuItem label="Employees" href="\#" icon_html={html!{<i class="fas fa-briefcase"></i>}} selected={selected_label.clone()} />
                <MenuItem label="Payroll" href="\#" icon_html={html!{<i class="fas fa-file-invoice-dollar"></i>}} selected={selected_label.clone()} />
            </Menu>
        </Sidebar>
    }
}"# }
                    </pre>
                    <Example6 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "DevOps Style" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use sidebar::yew::item::MenuItem;
use sidebar::yew::sidebar::Sidebar;
use sidebar::yew::menu::Menu;


#[function_component(Example7)]
pub fn example7() -> Html {
    let selected_label = use_state(|| "".to_string());
    html! {
        <Sidebar
            user_name="Ferris"
            designation="DevOps"
            user_img="/assets/logo.webp"
            style="background: #0f172a;"
            class="text-white transition duration-700 ease-in-out transform hover:translate-x-1"
        >
            <Menu sub_heading="OPS">
                <MenuItem label="Pipelines" href="\#" icon_html={html!{<i class="fas fa-code-branch"></i>}} selected={selected_label.clone()} />
                <MenuItem label="Deployments" href="\#" icon_html={html!{<i class="fas fa-server"></i>}} selected={selected_label.clone()} />
            </Menu>
        </Sidebar>
    }
}"# }
                    </pre>
                    <Example7 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Feature Flags" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use sidebar::yew::item::MenuItem;
use sidebar::yew::sidebar::Sidebar;
use sidebar::yew::menu::Menu;


#[function_component(Example8)]
pub fn example8() -> Html {
    let selected_label = use_state(|| "".to_string());
    html! {
        <Sidebar
            user_name="Ferris"
            designation="Product Owner"
            user_img="/assets/logo.webp"
            logo_img_url="/assets/logo.webp"
        >
            <Menu sub_heading="FEATURE FLAGS">
                <MenuItem label="New UI" href="\#" icon_html={html!{<i class="fas fa-toggle-on"></i>}} selected={selected_label.clone()} badge={Some("On")} />
                <MenuItem label="Beta Tools" href="\#" icon_html={html!{<i class="fas fa-tools"></i>}} selected={selected_label.clone()} badge={Some("Test")} />
            </Menu>
        </Sidebar>
    }
}"# }
                    </pre>
                    <Example8 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Marketing Panel" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use sidebar::yew::item::MenuItem;
use sidebar::yew::sidebar::Sidebar;
use sidebar::yew::menu::Menu;

#[function_component(Example9)]
pub fn example9() -> Html {
    let selected_label = use_state(|| "".to_string());
    html! {
        <Sidebar user_name="Ferris" designation="Marketing" user_img="/assets/logo.webp" style="width: 100%; background: linear-gradient(to left, #f9fafb, #d1d5db);">
            <Menu sub_heading="CAMPAIGNS">
                <MenuItem label="Ad Spend" href="\#" icon_html={html!{<i class="fas fa-bullhorn"></i>}} selected={selected_label.clone()} />
                <MenuItem label="Audience" href="\#" icon_html={html!{<i class="fas fa-users"></i>}} selected={selected_label.clone()} />
            </Menu>
        </Sidebar>
    }
}"# }
                    </pre>
                    <Example9 />
                </div>
            </div>
        </div>
    }
}
