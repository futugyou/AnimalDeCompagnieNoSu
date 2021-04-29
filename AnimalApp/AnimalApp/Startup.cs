using Microsoft.Maui;
using Microsoft.Maui.Hosting;
using Microsoft.Maui.Controls.Compatibility;
using Microsoft.Maui.LifecycleEvents;

namespace AnimalApp
{
	public class Startup : IStartup
	{
		public void Configure(IAppHostBuilder appBuilder)
		{
			appBuilder
				.UseFormsCompatibility()
				.UseMauiApp<App>()
				.ConfigureLifecycleEvents(lifecycle => {
#if ANDROID
                lifecycle.AddAndroid(d => { 
                    d.OnBackPressed(activity => { 
                        System.Diagnostics.Debug.WriteLine("Back button pressed!"); 
                    }); 
                }); 
#endif
				}); 
		}
	}
}