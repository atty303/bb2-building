# BB2 Building

A build planner for BB2 game.

## Development

### How to update the database

1. Install [Melon Loader](https://github.com/LavaGang/MelonLoader). (v0.6.2).
2. Install [Unity Explorer](https://github.com/yukieiji/UnityExplorer). (ML0.6/IL2CPP)
3. Run the game.
4. Run in C# Console.
   ```csharp
   using Il2CppBansheeGz.BGDatabase;
   ```
5. Run in C# Console.
   ```csharp
   BGJson.Export(BGRepo.I, "dump\\db.json");
   ```
6. Install [Asset Ripper](https://github.com/AssetRipper/AssetRipper).
7. Run Asset Ripper and select `File -> Open -> Open Folder`.
8. Select `Export -> Export All` and export to `dump/asset`.
