1. dotnet tool install -g redth.net.maui.check
2. maui-check
3. dotnet new maui -n AniamlApp
4. maui-check config --nuget-sources
5. build
```
dotnet build -t:Run -f net6.0-android
dotnet build -t:Run -f net6.0-ios
dotnet build -t:Run -f net6.0-maccatalyst
```
