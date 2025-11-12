#See https://aka.ms/containerfastmode to understand how Visual Studio uses this Dockerfile to build your images for faster debugging.

ARG NET_IMAGE=10.0
FROM mcr.microsoft.com/dotnet/aspnet:${NET_IMAGE} AS base
WORKDIR /app
EXPOSE 80

FROM mcr.microsoft.com/dotnet/sdk:${NET_IMAGE} AS build
WORKDIR /src
COPY ["Adoption.DbMigrator/Adoption.DbMigrator.csproj", "Adoption.DbMigrator/"]
COPY ["Adoption.Infrastruct.Data.DbMigrations/Adoption.Infrastruct.Data.DbMigrations.csproj", "Adoption.Infrastruct.Data.DbMigrations/"]
COPY ["Adoption.Infrastruct.Data/Adoption.Infrastruct.Data.csproj", "Adoption.Infrastruct.Data/"]
COPY ["Adoption.Domain/Adoption.Domain.csproj", "Adoption.Domain/"]
COPY ["Adoption.Domain.Shared/Adoption.Domain.Shared.csproj", "Adoption.Domain.Shared/"]
COPY ["Directory.Build.props", "./"]
COPY ["Directory.Packages.props", "./"]
RUN dotnet restore "Adoption.DbMigrator/Adoption.DbMigrator.csproj"
COPY . .
WORKDIR "/src/Adoption.DbMigrator"
# RUN dotnet build "Adoption.DbMigrator.csproj" -c Release -o /app/build

FROM build AS publish
RUN dotnet publish --no-restore "Adoption.DbMigrator.csproj" -c Release -o /app/publish

FROM base AS final
WORKDIR /app
COPY --from=publish /app/publish .
ENTRYPOINT ["dotnet", "Adoption.DbMigrator.dll"]