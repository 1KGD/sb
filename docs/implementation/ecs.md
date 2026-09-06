# ECS

> [!CAUTION]
> If you don't yet know what an ECS is, you should figure that out first (*hint: it stands for Entity Component System, try putting that into a search engine*) before reading the rest of this page. I promise you will be very confused otherwise.

Despite not using [bevy](https://bevy.org) for this project (*way* overkill), I have opted to use `bevy_ecs`. It is the most elegant and intuitive ECS I have worked with so far (I'm looking at you, `specs`). The vast majority of the game and underlying engine operates within it, with the exception of the mainloop handler (which actually triggers the systems), and the inital intro screens.