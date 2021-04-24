1. Install-Package  Volo.Abp.AspNetCore
2. add AdoptionWebModule, add DependsOn and impl AbpModule
3. override ConfigureServices and OnApplicationInitialization
4. move code from startup to AdoptionWebModule
5. fill ConfigureServices and Configure in startup
6. Install-Package Volo.Abp.Autofa and Volo.Abp.Swashbuckle