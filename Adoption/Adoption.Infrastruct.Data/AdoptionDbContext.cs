using Adoption.Domain.Adoption; 
using Microsoft.EntityFrameworkCore;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
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
