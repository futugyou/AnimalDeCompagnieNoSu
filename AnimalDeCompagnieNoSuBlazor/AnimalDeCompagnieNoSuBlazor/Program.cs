using AnimalDeCompagnieNoSuBlazor.Models;
using AnimalDeCompagnieNoSuBlazor.Services;
using AntDesign.Pro.Layout;
using Microsoft.AspNetCore.Components.WebAssembly.Hosting;
using Microsoft.Extensions.DependencyInjection;
using System;
using System.Net.Http;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor
{
    public class Program
    {
        public static async Task Main(string[] args)
        {
            var builder = WebAssemblyHostBuilder.CreateDefault(args);
            builder.RootComponents.Add<App>("#app");

            builder.Services.AddScoped(sp => new HttpClient { BaseAddress = new Uri(builder.HostEnvironment.BaseAddress) });
            builder.Services.AddAntDesign();
            AntDesign.LocaleProvider.SetLocale("zh-CN");
            builder.Services.Configure<ProSettings>(builder.Configuration.GetSection("ProSettings"));
            builder.Services.Configure<AnimalCenter>(builder.Configuration.GetSection("AnimalCenter"));
            builder.Services.AddHttpClient("AnimalCenter",
                client =>
                {
                    client.BaseAddress = new Uri(builder.Configuration["AnimalCenter:Host"]);
                    client.DefaultRequestHeaders.Add(builder.Configuration["AnimalCenter:HttpHeadKey"], builder.Configuration["AnimalCenter:HttpHeadValue"]);
                });
            builder.Services.AddScoped<IAnimalService, AnimalService>();
            builder.Services.AddScoped<IAnimalTypeService, AnimalTypeService>();
            builder.Services.AddScoped<IAnimalEventService, AnimalEventService>();
            builder.Services.AddScoped<IRescueService, RescueService>();
            await builder.Build().RunAsync();
        }
    }
}
