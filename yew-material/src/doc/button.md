按钮组件，doc 详见：https://www.yew-material.cn/zh/button

### Props | Name: Type[Default] - Description

    * icon: string['']
        - Icon to display, and aria-label value when label is not defined.
    * raised: boolean[false]
        - Creates a contained button that is elevated above the surface.
    * unelevated: boolean[false]
        - Creates a contained button that is flush with the surface.
    * outlined: boolean[false]
        - Creates an outlined button that is flush with the surface.
    * dense: boolean[false]
        - Makes the button text and container slightly smaller.
    * disabled: boolean[false]
        - Disabled buttons cannot be interacted with and have no visual interaction effect.
    * trailing_icon: boolean[false]
        - When true, icon will be displayed after label.
    * full_width: boolean[false]
        - When true, The width of the button is the same as the width of the parent node.

### Example

```rust
    html! {
        <Button
            class="..."
            raised=true
            icon="code"
            onclick=self.link.callback(...)
            ...
        >
            {"button"}
        </Button>
    }
```
