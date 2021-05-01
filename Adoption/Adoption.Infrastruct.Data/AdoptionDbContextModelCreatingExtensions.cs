using Adoption.Domain;
using Adoption.Domain.Adoption; 
using Microsoft.EntityFrameworkCore;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Volo.Abp;
using Volo.Abp.EntityFrameworkCore.Modeling;

namespace Adoption.Infrastruct.Data
{
    public static class AdoptionDbContextModelCreatingExtensions
    {
        public static void ConfigureAdoption(this ModelBuilder builder)
        {
            Check.NotNull(builder, nameof(builder));

            //builder.Entity<Animal>(b =>
            //{
            //    b.ToTable(AdoptionConsts.DbTablePrefix + "animals", AdoptionConsts.DbSchema);
            //    b.ConfigureByConvention();
            //    b.Property(x => x.Name).IsRequired().HasMaxLength(128);
            //    b.Property(x => x.CardId).IsRequired().HasMaxLength(128);
            //});
        }
    }
}
