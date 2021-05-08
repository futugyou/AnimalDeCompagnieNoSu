using Adoption.Application.Contracts;
using Microsoft.Extensions.DependencyInjection;
using Polly;
using System;
using Volo.Abp.Http.Client;
using Volo.Abp.Modularity;

namespace Adoption.API.Client
{
    /// <summary>
    /// I do not know the purpose of "Dynamic C# API Client Proxies".
    /// </summary>
    [DependsOn(
        typeof(AbpHttpClientModule),
        typeof(AdoptionApplicationContractsMdoule))]
    public class AdoptionAPIClientModule : AbpModule
    {
        public override void PreConfigureServices(ServiceConfigurationContext context)
        {
            PreConfigure<AbpHttpClientBuilderOptions>(options =>
            {
                options.ProxyClientBuildActions.Add((remoteServiceName, clientBuilder) =>
                {
                    clientBuilder.AddTransientHttpErrorPolicy(policyBuilder =>
                        policyBuilder.WaitAndRetryAsync(
                            3,
                            i => TimeSpan.FromSeconds(Math.Pow(2, i))
                        )
                    );
                });
            });
        }

        public override void ConfigureServices(ServiceConfigurationContext context)
        {
            //Create dynamic client proxies
            context.Services.AddHttpClientProxies(
                typeof(AdoptionApplicationContractsMdoule).Assembly
            );

            context.Services.Configure<AbpRemoteServiceOptions>(options =>
            {
                //options.RemoteServices.Default =
                //    new RemoteServiceConfiguration("http://localhost:53929/");
            });
        }
    }
}
