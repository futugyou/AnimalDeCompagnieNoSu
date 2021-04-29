using Microsoft.Maui;

namespace AnimalApp
{
	public class MainWindow : IWindow
	{
		public MainWindow()
		{
			Page = new MainPage();
		}

		public IPage Page { get; set; }

		public IMauiContext MauiContext { get; set; }
	}
}