using AntDesign;
using Microsoft.AspNetCore.Components;
using Microsoft.AspNetCore.Components.Web;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor
{
    public partial class AnimalEventDrawer : AntDomComponentBase
    {
        [Parameter]
        public bool Visible
        {
            get => _show;
            set => this._show = value;
        }
        [Parameter]
        public bool MaskClosable
        {
            get => _closable;
            set => this._closable = value;
        }
        private bool _show;
        private bool _closable;
        private void _onClose()
        {
             
        }

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
