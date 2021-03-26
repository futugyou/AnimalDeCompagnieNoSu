using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Extensions
{
    public class Tools
    {
        /// <summary>
        /// ant source code use this way to check a file is it a image
        /// </summary>
        /// <param name="fileName"></param>
        /// <returns></returns>
        public static bool IsPicture(string fileName)
        {
            if (fileName.LastIndexOf('.') == -1)
            {
                return false;
            }
            string[] imageTypes = new[] { ".jpg", ".png", ".gif", ".ico" };
            string ext = fileName[fileName.LastIndexOf('.')..];
            return imageTypes.Contains(ext);
        }
    }
}
