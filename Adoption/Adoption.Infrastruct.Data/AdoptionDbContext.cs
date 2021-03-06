﻿using Adoption.Domain.Adoption.Aggregate;
using Microsoft.EntityFrameworkCore;
using Volo.Abp.Data;
using Volo.Abp.EntityFrameworkCore;

namespace Adoption.Infrastruct.Data
{
    [ConnectionStringName("Default")]
    public class AdoptionDbContext : AbpDbContext<AdoptionDbContext>
    {
        public DbSet<AdoptionInfo> AdoptionInfos { get; set; }
        public AdoptionDbContext(DbContextOptions<AdoptionDbContext> options) : base(options)
        {

        }

        protected override void OnModelCreating(ModelBuilder modelBuilder)
        {
            base.OnModelCreating(modelBuilder);
            modelBuilder.ConfigureAdoption();
        }
    }
}
