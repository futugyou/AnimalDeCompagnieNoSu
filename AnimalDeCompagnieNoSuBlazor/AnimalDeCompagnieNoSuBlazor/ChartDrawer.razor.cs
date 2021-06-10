using AntDesign;
using Microsoft.AspNetCore.Components.Web;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor
{
    public partial class ChartDrawer : AntDomComponentBase
    {
        private bool _show;

        protected override void OnInitialized()
        {
            base.OnInitialized();

        }
        private void SetShow(MouseEventArgs args)
        {
            _show = !_show;
        }

    }
}
