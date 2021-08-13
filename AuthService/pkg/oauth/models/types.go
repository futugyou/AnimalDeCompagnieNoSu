package models

type Client struct {
	ID     string   `json:"id,omitempty"`
	Secret string   `json:"secret,omitempty"`
	Domain string   `json:"domain,omitempty"`
	UserID string   `json:"userID,omitempty"`
	Scopes []string `json:"scopes,omitempty"`
}

func (c *Client) GetID() string {
	return c.ID
}

func (c *Client) GetSecret() string {
	return c.Secret
}

func (c *Client) GetDomain() string {
	return c.Domain
}

func (c *Client) GetUserID() string {
	return c.UserID
}

func (c *Client) GetScopes() []string {
	return c.Scopes
}
