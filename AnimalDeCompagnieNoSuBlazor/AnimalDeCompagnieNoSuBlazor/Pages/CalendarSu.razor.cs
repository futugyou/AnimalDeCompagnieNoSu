using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Pages
{
    public partial class CalendarSu
    {
        protected override async Task OnInitializedAsync()
        {
            await base.OnInitializedAsync();
        }
        private void OnPanelChange(DateTime value, string mode)
        {
        }
        Random random = new Random(DateTime.Now.Millisecond);
        private IEnumerable<(string type, string content)> GetListData(DateTime dateTime)
        {
            var f = random.Next(0, 32);
            if (f > dateTime.Day)
            {
                yield return (dateTime.Month + "", dateTime.Day + "");
            }
        }
    }
}
