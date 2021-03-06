﻿using Adoption.Application;
using Adoption.Domain.Shared;
using Adoption.Infrastruct.Data;
using Microsoft.AspNetCore.Builder;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using Microsoft.OpenApi.Models;
using System;
using System.IO;
using Volo.Abp;
using Volo.Abp.AspNetCore;
using Volo.Abp.AspNetCore.Mvc;
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
        typeof(AdoptionApplicationModule),
        typeof(AdoptionInfrastructDataModule))]
    public class AdoptionHostModule : AbpModule
    {
        public override void ConfigureServices(ServiceConfigurationContext context)
        {
            var services = context.Services;
            var _configuration = context.Services.GetConfiguration();
            ConfigureLocalization();
            ConfigureVirtualFileSystem(context);
            services.AddControllers();
            ConfigureSwaggerServices(context);
            ConfigureAutoAPIControllers();
        }

        private void ConfigureAutoAPIControllers()
        {
            Configure<AbpAspNetCoreMvcOptions>(options =>
            {
                options.ConventionalControllers
                    .Create(typeof(AdoptionApplicationModule).Assembly, opts =>
                    {
                        // Can not be "" or "/" .
                        // The route template separator character '/' cannot appear consecutively.
                        // It must be separated by either a parameter or a literal value.
                        //opts.RootPath = "/";
                    });
            });
        }

        private void ConfigureLocalization()
        {
            Configure<AbpLocalizationOptions>(options =>
            {
                options.Languages.Add(new LanguageInfo("en", "en", "English"));
                options.Languages.Add(new LanguageInfo("zh", "zh", "Chinese"));
            });
        }

        private void ConfigureVirtualFileSystem(ServiceConfigurationContext context)
        {
            var hostingEnvironment = context.Services.GetHostingEnvironment();

            if (hostingEnvironment.IsDevelopment())
            {
                Configure<AbpVirtualFileSystemOptions>(options =>
                {
                    options.FileSets.ReplaceEmbeddedByPhysical<AdoptionDomainSharedModule>(
                    Path.Combine(hostingEnvironment.ContentRootPath,
                        $"..{Path.DirectorySeparatorChar}Adoption.Domain.Shared"));
                });
            }
        }

        private static void ConfigureSwaggerServices(ServiceConfigurationContext context)
        {
            context.Services.AddSwaggerGen(
                options =>
                {
                    options.SwaggerDoc("v1", new OpenApiInfo { Title = "AdoptionCenter", Version = "v1" });
                    options.DocInclusionPredicate((docName, description) => true);
                });
        }

        public override void OnApplicationInitialization(ApplicationInitializationContext context)
        {
            var app = context.GetApplicationBuilder();
            var env = context.GetEnvironment();
            if (env.IsDevelopment())
            {
                app.UseDeveloperExceptionPage();
            }
            app.UseAbpRequestLocalization();
            app.UseHttpsRedirection();

            app.UseStaticFiles();
            //app.UseVirtualFiles();

            app.UseRouting();
            app.UseAuthorization();

            app.UseSwagger();
            app.UseSwaggerUI(c => c.SwaggerEndpoint("/swagger/v1/swagger.json", "AdoptionCenter v1"));

            app.UseEndpoints(endpoints =>
            {
                endpoints.MapControllers();
            });
        }
    }
}
