package generates

import (
	"bytes"
	"context"
	"encoding/base64"
	"strings"

	"github.com/go-oauth2/oauth2/v4"
	"github.com/google/uuid"
)

// NewCustomAuthorize create to generate the authorize code instance
func NewCustomAuthorize() *CustomAuthorize {
	return &CustomAuthorize{}
}

// CustomAuthorize generate the authorize code
type CustomAuthorize struct{}

// Token based on the UUID generated token
func (ag *CustomAuthorize) Token(ctx context.Context, data *oauth2.GenerateBasic) (string, error) {
	buf := bytes.NewBufferString(data.Client.GetID())
	buf.WriteString(data.UserID)
	buf.WriteString(data.TokenInfo.GetScope())
	token := uuid.NewMD5(uuid.Must(uuid.NewRandom()), buf.Bytes())
	code := base64.URLEncoding.EncodeToString([]byte(token.String()))
	code = strings.ToUpper(strings.TrimRight(code, "="))

	return code, nil
}
