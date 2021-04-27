###### integrate abp vNext with empty aspcnetcore web api project. Current Abp Version 4.3.0
1. Create an empty web api project, called Adoption.Host.
2. Install-Package Volo.Abp.AspNetCore/Volo.Abp.Swashbuckle.
3. Create AdoptionHostModule impl AbpModule, add AbpAspNetCoreModule and AbpSwashbuckleModule DependsOnAttribute.
4. Override ConfigureServices and OnApplicationInitialization
5. Move code from startup to AdoptionHostModule, fill ConfigureServices and Configure in startup

###### Add Domain to sln
1. Create an lib project, called Adoption.Domain.
2. Install-Package Volo.Abp.AspNetCore/Volo.Abp.Ddd.Domain.
3. Create AdoptionDomainModule impl AbpModule, add AbpDddDomainModule DependsOnAttribute.
4. Create a Domain (e.g Animals) impl Entity<T> or AggregateRoot<T> or others.

###### Add Infrastruct project (e.g.  dbcontext) to sln
1. Create an lib project, called Adoption.Infrastruct.Data.
2. Install-Package Microsoft.EntityFrameworkCore.Design to Adoption.Host. (Use ef5.0.5, becase ef6.0-preview have some problem now. 2021/4/26)
3. Install-Package Microsoft.EntityFrameworkCore.Tools to Adoption.Infrastruct.Data.
4. Install-Package Volo.Abp.AspNetCore/Volo.Abp.EntityFrameworkCore/Volo.Abp.EntityFrameworkCore.MySQL(or other) to Adoption.Infrastruct.Data.
5. Create AdoptionInfrastructDataModule impl AbpModule, add AdoptionDomainModule and AbpEntityFrameworkCoreModule DependsOnAttribute.
6. Create DbContext impl AbpDbContext<DbContext> , add DbSet<Domain>, and you can override base methods if you need.
7. AddAbpDbContext<DbContext> and Configure<AbpDbContextOptions> in AdoptionInfrastructDataModule ConfigureServices method.

###### Add Application Contracts to sln
1. Create an lib project, called Adoption.Application.Contracts.
2. Install-Package Volo.Abp.Ddd.Application.Contracts.
3. Create Dto  (e.g AnimalDto) impl EntityDto<T>.
4. Create AdoptionApplicationContractsMdoule impl AbpModule.

###### Add Application to sln
1. Create an lib project, called Adoption.Application.
2. Install-Package Volo.Abp.AspNetCore/Volo.Abp.AutoMapper/Volo.Abp.Ddd.Application.
3. Create AutoMapperProfile impl Profile.
4. Create service (interface and impl) inherit IApplicationService and ApplicationService.
5. Create AdoptionApplicationModule impl AbpModule, add  AbpDddApplicationModule/AbpAutoMapperModule/AdoptionInfrastructDataModule/AdoptionApplicationContractsMdoule DependsOnAttribute.
6. Configure<AbpAutoMapperOptions> in order to load all AutoMapperProfile in this module.
7. Add AdoptionApplicationModule DependsOnAttribute to AdoptionHostModule.

###### Others
1. mysql
```
docker run -p 127.0.0.1:3306:3306  --name  mariadb -e MYSQL_ROOT_PASSWORD=123456 -d mariadb
```
2. mirage database
```
add-migration init -Context AdoptionDbContext
update-database -Context AdoptionDbContext
```
3. debug in docker 
https://code.visualstudio.com/docs/containers/quickstart-aspnet-core