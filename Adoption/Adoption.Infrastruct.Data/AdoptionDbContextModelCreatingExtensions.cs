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

            builder.Entity<Adopter>(b =>
            {
                b.ToTable(AdoptionConsts.DbTablePrefix + "adopters", AdoptionConsts.DbSchema);
                b.ConfigureByConvention();

                b.OwnsOne(p => p.Address, ownedBuilder =>
                {
                    ownedBuilder.Property(x => x.Province).IsRequired().HasMaxLength(20);
                    ownedBuilder.Property(x => x.City).IsRequired().HasMaxLength(20);
                    ownedBuilder.Property(x => x.Street).IsRequired().HasMaxLength(40);
                    ownedBuilder.Property(x => x.Home).IsRequired().HasMaxLength(20);
                });
                b.Navigation(p => p.Address).IsRequired();

                b.Property(x => x.Name).IsRequired().HasMaxLength(128);
                b.Property(x => x.IdNumber).IsRequired().HasMaxLength(20);
                b.Property(x => x.Phone).IsRequired().HasMaxLength(20);
            });

            builder.Entity<Animal>(b =>
            {
                b.ToTable(AdoptionConsts.DbTablePrefix + "animals", AdoptionConsts.DbSchema);
                b.ConfigureByConvention();
                b.Property(x => x.Name).IsRequired().HasMaxLength(128);
                b.Property(x => x.CardId).IsRequired().HasMaxLength(28);
            });


            builder.Entity<AdoptionInfo>(b =>
            {
                b.ToTable(AdoptionConsts.DbTablePrefix + "adoption_infos", AdoptionConsts.DbSchema);
                b.ConfigureByConvention();
                b.Property(x => x.AdoptionReasons).IsRequired().HasDefaultValue("").HasMaxLength(400);
                b.Property(x => x.AdoptionResult).IsRequired().HasDefaultValue("").HasMaxLength(400);

                b.Navigation(p => p.Animal).IsRequired();
                b.Navigation(p => p.Adopter).IsRequired();
            });
        }
    }
}
