package oauth

import (
	"io"
	"log"
	"net/http"
	"net/http/httputil"
	"os"

	"github.com/go-oauth2/oauth2/v4/errors"
	"github.com/go-oauth2/oauth2/v4/generates"
	"github.com/go-oauth2/oauth2/v4/manage"
	"github.com/go-oauth2/oauth2/v4/models"
	"github.com/go-oauth2/oauth2/v4/server"
	"github.com/go-oauth2/oauth2/v4/store"
	"github.com/go-session/session"
	"github.com/golang-jwt/jwt"
)

func Init() *server.Server {
	manager := manage.NewDefaultManager()
	// set default code ttl
	manager.SetAuthorizeCodeTokenCfg(manage.DefaultAuthorizeCodeTokenCfg)
	// token store
	manager.MustTokenStorage(store.NewMemoryTokenStore())

	// generate jwt access token
	manager.MapAccessGenerate(generates.NewJWTAccessGenerate("", []byte("00000000"), jwt.SigningMethodHS512))
	//manager.MapAccessGenerate(generates.NewAccessGenerate())

	clientStore := store.NewClientStore()
	clientStore.Set("000000", &models.Client{
		ID:     "000000",
		Secret: "999999",
		Domain: "http://localhost:8080",
	})
	manager.MapClientStorage(clientStore)

	srv := server.NewServer(server.NewConfig(), manager)

	srv.SetAllowGetAccessRequest(true)
	srv.SetClientInfoHandler(server.ClientFormHandler)

	srv.SetPasswordAuthorizationHandler(passwordAuthorizationHandler)
	srv.SetUserAuthorizationHandler(userAuthorizeHandler)

	srv.SetInternalErrorHandler(internalErrorHandler)
	srv.SetResponseErrorHandler(responseErrorHandler)
	return srv
}

func responseErrorHandler(re *errors.Response) {
	log.Println("Response Error: ", re.Description)
}

func internalErrorHandler(err error) (re *errors.Response) {
	log.Println("Internal Error: ", err.Error())
	return
}

func passwordAuthorizationHandler(username, password string) (userID string, err error) {
	if username == "test" && password == "test" {
		userID = "test"
	}
	return
}

func userAuthorizeHandler(w http.ResponseWriter, r *http.Request) (userID string, err error) {

	_ = dumpRequest(os.Stdout, "userAuthorizeHandler", r) // Ignore the error

	store, err := session.Start(r.Context(), w, r)
	if err != nil {
		return
	}

	uid, ok := store.Get("LoggedInUserID")
	if !ok {
		if r.Form == nil {
			r.ParseForm()
		}

		store.Set("ReturnUri", r.Form)
		store.Save()

		w.Header().Set("Location", "/login")
		w.WriteHeader(http.StatusFound)
		return
	}

	userID = uid.(string)
	store.Delete("LoggedInUserID")
	store.Save()
	return
}

func dumpRequest(writer io.Writer, header string, r *http.Request) error {
	data, err := httputil.DumpRequest(r, true)
	if err != nil {
		return err
	}
	writer.Write([]byte("\n" + header + ": \n"))
	writer.Write(data)
	return nil
}
