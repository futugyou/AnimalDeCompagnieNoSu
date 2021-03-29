using AnimalDeCompagnieNoSuBlazor.Models;
using AnimalDeCompagnieNoSuBlazor.Pages.Animal;
using AntDesign;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace AnimalDeCompagnieNoSuBlazor.Extensions
{
    public class TypeConvertTools
    {
        public static List<CascaderNode> AnimalTypeToCascaderNode(List<AnimalType> animalTypes)
        {
            List<CascaderNode> nodelist = new List<CascaderNode>();
            nodelist.AddRange(animalTypes.Where(p => p.Pid == "")
                .Select(p => new CascaderNode() { Value = p.Id, Label = p.Type, Children = new List<CascaderNode>() }));

            foreach (var item in animalTypes.Where(p => p.Pid != ""))
            {
                CascaderNode node = new CascaderNode() { Value = item.Id, Label = item.Type, Children = new List<CascaderNode>() };
                var tmp = GetCascaderNode(nodelist, item.Pid);
                if (tmp != null)
                {
                    tmp.Children = tmp.Children.Append(node).ToList();
                }
            }
            ChangeNodeValue(nodelist);
            return nodelist;
        }

        private static void ChangeNodeValue(IEnumerable<CascaderNode> cascaderNodes)
        {
            if (cascaderNodes == null || !cascaderNodes.Any())
            {
                return;
            }
            foreach (var item in cascaderNodes)
            {
                item.Value = item.Label;
            }
            var subnodes = cascaderNodes.SelectMany(p => p.Children);
            ChangeNodeValue(subnodes);
        }

        private static CascaderNode GetCascaderNode(List<CascaderNode> cascaderNodes, string pid)
        {
            var tmp = cascaderNodes.FirstOrDefault(p => p.Value == pid);
            if (tmp != null)
            {
                return tmp;
            }
            return GetCascaderNode(cascaderNodes.SelectMany(p => p.Children).ToList(), pid);
        }

        internal static List<SelectType> AnimalTypeToSelectType(List<AnimalType> animalTypes)
        {
            var selectList = animalTypes.Where(p => string.IsNullOrEmpty(p.Pid))
                .Select(p => new SelectType { Value = p.Type, Text = p.Type, Group = p.Pid }).ToList();
            foreach (var item in animalTypes.Where(p => p.Pid != ""))
            {
                var parent = animalTypes.FirstOrDefault(p => p.Id == item.Pid);
                selectList.Add(new SelectType { Value = item.Type, Text = item.Type, Group = parent.Type });
            }
            return selectList;
        }
    }
}
