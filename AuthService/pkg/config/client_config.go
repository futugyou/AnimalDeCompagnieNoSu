package config

type AuthClient struct {
	ID     string
	Secret string
	Domain string
	UserID string
	Scopes []string
}
