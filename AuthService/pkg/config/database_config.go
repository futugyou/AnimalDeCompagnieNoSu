package config

import (
	"fmt"
)

type database struct {
	Host     string
	Port     int
	User     string
	Password string
	Dbname   string
}

func (db *database) ToConnectionString() string {
	return fmt.Sprintf("host=%s port=%d user=%s password=%s dbname=%s sslmode=disable",
		db.Host,
		db.Port,
		db.User,
		db.Password,
		db.Dbname,
	)
}
