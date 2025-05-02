# Y Sidebar Yew Usage

Adding Sidebar to your project is simple:

1. Make sure your project is set up with **Yew**. Follow their [Getting Started Guide](https://yew.rs/docs/getting-started/introduction) for setup instructions.

1. Add the Sidebar component to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add sidebar --features=yew
   ```

1. Import the `Sidebar` component into your Yew component and start using it in your app.

## 🛠️ Usage

Integrating the **Sidebar** into your Yew application is straightforward. Follow the steps below:

1. Import the `Sidebar`, `Menu`, `MenuItem`, `Submenu`, and `Profile` components:

   ```rust
   use yew::prelude::*;
   use sidebar::yew::item::MenuItem;
   use sidebar::yew::menu::Menu;
   use sidebar::yew::sidebar::Sidebar;
   use sidebar::yew::submenu::Submenu;
   ```

1. Use them in your application:

   ```rust
   use yew::prelude::*;
   use sidebar::yew::item::MenuItem;
   use sidebar::yew::menu::Menu;
   use sidebar::yew::sidebar::Sidebar;
   use sidebar::yew::submenu::Submenu;

   #[function_component(App)]
   pub fn app() -> Html {
       let selected = use_state(|| String::from("Dashboard"));

       html! {
           <Sidebar
               user_name="Ferris"
               designation="Admin"
               user_img="/assets/logo.webp"
               on_logout={Callback::from(|_| log::info!("Logout!"))}
               logo_img_url="/logo.svg"
               logo_href="/"
           >
               <Menu sub_heading="Main">
                   <MenuItem
                       label="Dashboard"
                       href="/dashboard"
                       icon_html={html! {<span>{ "📊" }</span>}}
                       selected={selected.clone()}
                   />
                   <MenuItem
                       label="Settings"
                       href="/settings"
                       icon_html={html! {<span>{ "⚙️" }</span>}}
                       selected={selected.clone()}
                   />
                   <Submenu title="More" icon_html={html! {<span>{ "➕" }</span>}}>
                       <MenuItem
                           label="Reports"
                           href="/reports"
                           icon_html={html! {<span>{ "📁" }</span>}}
                           selected={selected.clone()}
                       />
                   </Submenu>
               </Menu>
           </Sidebar>
       }
   }
   ```

## 🔧 Props

```sh
+---------------------------------------------------------------+
|                          Sidebar                              |
|  (style: width: 270px; background: white;)                    |
|                                                               |
|  +---------------------------------------------------------+  |
|  |                     Header (Logo + Toggle)              |  |
|  |   (header_style: display: flex; align-items: center;)   |  |
|  |                                                         |  |
|  |   [Logo]                             [Toggle ◀ or ▶]    |  |
|  +---------------------------------------------------------+  |
|                                                               |
|  +---------------------------------------------------------+  |
|  |                        Menu                             |  |
|  |  (sub_heading shown if not collapsed)                   |  |
|  |                                                         |  |
|  |   +-------------------+   +-------------------+         |  |
|  |   |    MenuItem       |   |    MenuItem       |         |  |
|  |   |  [Icon] Label     |   |  [Icon] Label     |         |  |
|  |   +-------------------+   +-------------------+         |  |
|  |                                                         |  |
|  |   +-------------------+                                 |  |
|  |   |     Submenu       |                                 |  |
|  |   |  [Icon] Label  ▼  |(click to expand ▼ or collapse ▲)|  |
|  |   +-------------------+                                 |  |
|  |       |                                                 |  |
|  |       |--> +-------------------+                        |  |
|  |            |    MenuItem       |                        |  |
|  |            |  [Icon] Label     |                        |  |
|  |            +-------------------+                        |  |
|  +---------------------------------------------------------+  |
|                                                               |
|  +---------------------------------------------------------+  |
|  |                       Profile                           |  |
|  |  (shown only if not collapsed)                          |  |
|  |                                                         |  |
|  |  [Avatar] Username        Designation          [-]      |  |
|  +---------------------------------------------------------+  |
+---------------------------------------------------------------+
```

### `Sidebar` Component Props

| Property       | Type                      | Description                               | Default                              |
| -------------- | ------------------------- | ----------------------------------------- | ------------------------------------ |
| `children`     | `ChildrenWithProps<Menu>` | The `Menu` components rendered in sidebar | `[]`                                 |
| `show_profile` | `bool`                    | Whether to show the profile section       | `true`                               |
| `user_name`    | `&'static str`            | User's name displayed in profile          | `""`                                 |
| `designation`  | `&'static str`            | User role/designation                     | `""`                                 |
| `user_img`     | `&'static str`            | Path to user image                        | `""`                                 |
| `on_logout`    | `Callback<()>`            | Callback invoked when logout is clicked   | `No-op`                              |
| `style`        | `&'static str`            | Inline CSS for sidebar container          | `"width: 270px; background: white;"` |
| `class`        | `&'static str`            | CSS class for sidebar container           | `""`                                 |
| `header_style` | `&'static str`            | CSS for the sidebar header section        | `"display: flex; ..."`               |
| `header_class` | `&'static str`            | Class for the header div                  | `""`                                 |
| `logo_img_url` | `&'static str`            | Logo image path                           | `""`                                 |
| `logo_href`    | `&'static str`            | Logo link target                          | `""`                                 |

### `Menu` Component Props

| Property      | Type           | Description                      | Default            |
| ------------- | -------------- | -------------------------------- | ------------------ |
| `sub_heading` | `&'static str` | Optional section heading         | `""`               |
| `children`    | `Children`     | Children `MenuItem` or `Submenu` | `[]`               |
| `style`       | `&'static str` | Inline styles                    | `"padding: 1rem;"` |
| `class`       | `&'static str` | CSS class                        | `""`               |

### `MenuItem` Component Props

| Property                | Type                     | Description                             | Default                                      |
| ----------------------- | ------------------------ | --------------------------------------- | -------------------------------------------- |
| `label`                 | `&'static str`           | Menu item label                         | `""`                                         |
| `href`                  | `&'static str`           | Navigation URL                          | `""`                                         |
| `icon_html`             | `Html`                   | Icon HTML for the item                  | `html!{}`                                    |
| `badge`                 | `Option<&'static str>`   | Optional badge (e.g., "3", "new")       | `None`                                       |
| `selected`              | `UseStateHandle<String>` | Currently selected item state           | Required                                     |
| `anchor_style`          | `&'static str`           | Styles for anchor wrapper               | `"text-decoration: none;"`                   |
| `anchor_class`          | `&'static str`           | Class for anchor wrapper                | `""`                                         |
| `item_style`            | `&'static str`           | Styles when sidebar is expanded         | `"display: flex; ..."`                       |
| `item_class`            | `&'static str`           | Class for item container                | `""`                                         |
| `collapsed_style`       | `&'static str`           | Styles for collapsed sidebar            | `"display: flex; ..."`                       |
| `collapsed_label_style` | `&'static str`           | Styles for label text in collapsed mode | `"font-size: 10px; text-align: center;"`     |
| `selected_style`        | `&'static str`           | Styles for selected item                | `"background-color: #1e293b; color: white;"` |
| `badge_style`           | `&'static str`           | Style for badge                         | `"background: red; ..."`                     |

### `Submenu` Component Props

| Property    | Type                          | Description                          | Default                                 |
| ----------- | ----------------------------- | ------------------------------------ | --------------------------------------- |
| `title`     | `&'static str`                | Submenu title                        | `""`                                    |
| `icon_html` | `Html`                        | Optional icon                        | `html!{}`                               |
| `children`  | `ChildrenWithProps<MenuItem>` | Menu items to be shown when expanded | `[]`                                    |
| `class`     | `&'static str`                | CSS class                            | `""`                                    |
| `style`     | `&'static str`                | Inline style                         | `"padding: 10px; cursor: pointer; ..."` |

### `Profile` Component Props

| Property       | Type           | Description                        | Default                                     |
| -------------- | -------------- | ---------------------------------- | ------------------------------------------- |
| `user_name`    | `&'static str` | Name of the user                   | `""`                                        |
| `designation`  | `&'static str` | User's designation/role            | `""`                                        |
| `user_img`     | `&'static str` | Profile image path                 | `""`                                        |
| `is_collapsed` | `bool`         | Whether sidebar is collapsed       | `false`                                     |
| `on_logout`    | `Callback<()>` | Callback triggered on logout click | `No-op`                                     |
| `style`        | `&'static str` | Styling for the profile container  | `"display: flex; align-items: center; ..."` |
| `class`        | `&'static str` | Class for the container            | `""`                                        |

## 💡 Notes

- `Sidebar` handles responsive behavior based on viewport width (`< 768px` = collapsed).
- `MenuItem` auto-detects and highlights current item using the `selected` state and URL query param `selected`.
- You **must pass a shared `UseStateHandle<String>`** to `selected` in every `MenuItem` to track active selection.
- `Submenu` provides collapsible behavior to group nested items.
- `Profile` is automatically hidden in collapsed mode.
- Customize with `style` and `class` props for full design flexibility.
- `ContextProvider` enables state sharing (collapsed/expanded) across components via `SidebarContext`.
