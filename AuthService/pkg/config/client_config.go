package config

type AuthClient struct {
	ID     string
	Secret string
	Domain string
	UserID string
	Scopes []string
}

func (c *AuthClient) GetID() string {
	return c.ID
}

func (c *AuthClient) GetSecret() string {
	return c.Secret
}

func (c *AuthClient) GetDomain() string {
	return c.Domain
}

func (c *AuthClient) GetUserID() string {
	return c.UserID
}
func (c *AuthClient) GetScopes() []string {
	return c.Scopes
}
