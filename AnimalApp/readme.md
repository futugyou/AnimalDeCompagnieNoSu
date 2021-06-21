[Document](https://github.com/dotnet/maui-samples/#installing-with-official-preview-installers)
1. dotnet tool install -g redth.net.maui.check
2. maui-check
3. dotnet workload install microsoft-android-sdk-full
4. dotnet new maui -n AniamlApp
5. #maui-check config --nuget-sources
6. build
```
dotnet build -t:Run -f net6.0-android
dotnet build -t:Run -f net6.0-ios
dotnet build -t:Run -f net6.0-maccatalyst
```
