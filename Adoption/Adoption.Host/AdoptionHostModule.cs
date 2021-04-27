using Adoption.Application;
using Adoption.Application.Contracts;
using Microsoft.AspNetCore.Builder;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using Microsoft.OpenApi.Models;
using System.IO;
using Volo.Abp;
using Volo.Abp.AspNetCore;
using Volo.Abp.Autofac;
using Volo.Abp.Localization;
using Volo.Abp.Modularity;
using Volo.Abp.Swashbuckle;
using Volo.Abp.VirtualFileSystem;

namespace Adoption.Host
{
    [DependsOn(
        typeof(AbpAspNetCoreModule),
        typeof(AbpSwashbuckleModule),
        typeof(AbpAutofacModule),
        typeof(AdoptionApplicationModule))]
    public class AdoptionHostModule : AbpModule
    {
        public override void ConfigureServices(ServiceConfigurationContext context)
        {
            var services = context.Services;
            var configuration = context.Services.GetConfiguration();
            //services.AddRabbitMQConnection(configuration);
            // Add the Redis distributed cache.
            // We are using the Steeltoe Redis Connector to pickup the CloudFoundry
            // Redis Service binding and use it to configure the underlying RedisCache
            // This adds a IDistributedCache to the container
            // services.AddDistributedRedisCache(configuration);
            // This works like the above, but adds a IConnectionMultiplexer to the container
            // services.AddRedisConnectionMultiplexer(configuration);
            //services.AddMongoClient(configuration);
            //services.AddDbContext<TestContext>(options => options.UseNpgsql(configuration));
            Configure<AbpLocalizationOptions>(options =>
            {
                options.Languages.Add(new LanguageInfo("en", "en", "English"));
                options.Languages.Add(new LanguageInfo("zh", "zh", "Chinese"));
            });
            var hostingEnvironment = context.Services.GetHostingEnvironment();
            Configure<AbpVirtualFileSystemOptions>(options =>
            {
                options.FileSets.ReplaceEmbeddedByPhysical<AdoptionApplicationContractsMdoule>(
                    Path.Combine(hostingEnvironment.ContentRootPath,
                        $"..{Path.DirectorySeparatorChar}Adoption.Application.Contracts"));
            });
            services.AddControllers();

            services.AddSwaggerGen(c =>
            {
                c.SwaggerDoc("v1", new OpenApiInfo { Title = "AdoptionCenter", Version = "v1" });
            });
        }
        public override void OnApplicationInitialization(ApplicationInitializationContext context)
        {
            var app = context.GetApplicationBuilder();
            var env = context.GetEnvironment();
            if (env.IsDevelopment())
            {
                app.UseDeveloperExceptionPage();
                app.UseSwagger();
                app.UseSwaggerUI(c => c.SwaggerEndpoint("/swagger/v1/swagger.json", "AdoptionCenter v1"));
            }
            app.UseAbpRequestLocalization();
            app.UseHttpsRedirection();
            app.UseStaticFiles();
            app.UseRouting();

            app.UseAuthorization();

            app.UseEndpoints(endpoints =>
            {
                endpoints.MapControllers();
            });
        }
    }
}
