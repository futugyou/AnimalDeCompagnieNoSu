#See https://aka.ms/containerfastmode to understand how Visual Studio uses this Dockerfile to build your images for faster debugging.

FROM mcr.microsoft.com/dotnet/aspnet:6.0 AS base
WORKDIR /app
EXPOSE 80

FROM mcr.microsoft.com/dotnet/sdk:6.0 AS build
WORKDIR /src
COPY "Adoption.sln" "Adoption.sln"
COPY ["AdoptionApi/Adoption.csproj", "AdoptionApi/"]
COPY ["Adoption.Infrastruct.Data/Adoption.Infrastruct.Data.csproj", "Adoption.Infrastruct.Data/"]
RUN dotnet restore "Adoption.sln"
COPY . .
WORKDIR "/src/AdoptionApi"
#RUN dotnet build "Adoption.csproj" -c Release -o /app/build

FROM build AS publish
RUN dotnet publish "Adoption.csproj" -c Release -o /app/publish

FROM base AS final
WORKDIR /app
COPY --from=publish /app/publish .
ENTRYPOINT ["dotnet", "Adoption.dll"]