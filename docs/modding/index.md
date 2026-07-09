# Modding

> [!WARNING]
> All of this is highly speculative and most of these frameworks do not exist yet.

Ideally, STARBLOOM has first-class modding support. The ability to create new components, [tiles](./tiles.md), biomes, systems, etc., and have them interact with existing systems in a simple and intuitive manner is crucial.

Part of this *already* exists in the form of the tile/biome regestries build into the core of the game. So long as a function provided by the mod can be called between the addition of these regestries to the ECS and the kick-starting of the mainloop it should work flawlessly. In fact, the tile id system is *already* backwards-compatable (once save games are implemented) with adding/removing mods (the latter would still cause a crash as of right now, though).

:::{toctree}
:hidden: true
tiles
:::