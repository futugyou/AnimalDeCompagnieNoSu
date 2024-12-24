#See https://aka.ms/containerfastmode to understand how Visual Studio uses this Dockerfile to build your images for faster debugging.

ARG NET_IMAGE=9.0
FROM mcr.microsoft.com/dotnet/aspnet:${NET_IMAGE} AS base
WORKDIR /app
EXPOSE 80

FROM mcr.microsoft.com/dotnet/sdk:${NET_IMAGE} AS build
WORKDIR /src
COPY ["Adoption.Host/Adoption.Host.csproj", "Adoption.Host/"]
COPY ["Adoption.Application/Adoption.Application.csproj", "Adoption.Application/"]
COPY ["Adoption.Infrastruct.Data/Adoption.Infrastruct.Data.csproj", "Adoption.Infrastruct.Data/"]
COPY ["Adoption.Domain/Adoption.Domain.csproj", "Adoption.Domain/"]
COPY ["Adoption.Domain.Shared/Adoption.Domain.Shared.csproj", "Adoption.Domain.Shared/"]
COPY ["Adoption.Application.Contracts/Adoption.Application.Contracts.csproj", "Adoption.Application.Contracts/"]
RUN dotnet restore "Adoption.Host/Adoption.Host.csproj"
COPY . .
WORKDIR "/src/Adoption.Host"
# RUN dotnet build "Adoption.Host.csproj" -c Release -o /app/build

FROM build AS publish
RUN dotnet publish --no-restore "Adoption.Host.csproj" -c Release -o /app/publish

FROM base AS final
WORKDIR /app
COPY --from=publish /app/publish .
ENTRYPOINT ["dotnet", "Adoption.Host.dll"]