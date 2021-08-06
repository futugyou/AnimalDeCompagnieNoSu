package oauth

import (
	"context"
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

type OAuthHandler struct {
	*server.Server
}

func New() OAuthHandler {
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
	return OAuthHandler{
		Server: srv,
	}
}

func responseErrorHandler(re *errors.Response) {
	log.Println("Response Error: ", re.Description)
}

func internalErrorHandler(err error) (re *errors.Response) {
	log.Println("Internal Error: ", err.Error())
	return
}

// get user id from username and password
func passwordAuthorizationHandler(username, password string) (userID string, err error) {
	if username == "test" && password == "test" {
		userID = "test"
	}
	return
}

// get user id from request authorization
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

func (handler *OAuthHandler) AuthorizeHandler(w http.ResponseWriter, r *http.Request) {
	err := handler.HandleAuthorizeRequest(w, r)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
	}
}

func (handler *OAuthHandler) TokenHandler(w http.ResponseWriter, r *http.Request) {
	handler.HandleTokenRequest(w, r)
}

func (handler *OAuthHandler) LoginHandler(w http.ResponseWriter, r *http.Request) {
	_ = dumpRequest(os.Stdout, "login", r)
	store, err := session.Start(r.Context(), w, r)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	if r.Method == "POST" {
		if r.Form == nil {
			if err := r.ParseForm(); err != nil {
				http.Error(w, err.Error(), http.StatusInternalServerError)
				return
			}
		}
		store.Set("LoggedInUserID", r.Form.Get("username"))
		store.Save()
		w.Header().Set("Location", "/auth")
		w.WriteHeader(http.StatusFound)
		return
	}
}

func (handler *OAuthHandler) AuthHandler(w http.ResponseWriter, r *http.Request) {
	_ = dumpRequest(os.Stdout, "auth", r)
	store, err := session.Start(context.TODO(), w, r)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	if _, ok := store.Get("LoggedInUserID"); ok {
		w.Header().Set("Location", "/login")
		w.WriteHeader(http.StatusFound)
		return
	}
}

func outputHTML(w http.ResponseWriter, req *http.Request, filename string) {
	file, err := os.Open(filename)
	if err != nil {
		http.Error(w, err.Error(), 500)
		return
	}
	defer file.Close()
	fi, _ := file.Stat()
	http.ServeContent(w, req, file.Name(), fi.ModTime(), file)
}
