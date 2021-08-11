package oauth

import (
	"context"
	"encoding/json"
	"io"
	"log"
	"net/http"
	"net/http/httputil"
	"net/url"
	"os"
	"time"

	"github.com/futugyou/AnimalDeCompagnieNoSu/AuthService/pkg/config"
	"github.com/go-oauth2/oauth2/v4/errors"
	"github.com/go-oauth2/oauth2/v4/generates"
	"github.com/go-oauth2/oauth2/v4/manage"
	"github.com/go-oauth2/oauth2/v4/models"
	"github.com/go-oauth2/oauth2/v4/server"
	"github.com/go-session/session"
	"github.com/golang-jwt/jwt"
	"github.com/jackc/pgx/v4"
	pg "github.com/vgarvardt/go-oauth2-pg/v4"
	"github.com/vgarvardt/go-pg-adapter/pgx4adapter"
)

type OAuthHandler struct {
	*server.Server
}

func New() OAuthHandler {
	psqlInfo := config.DatabaseSetting.ToConnectionString()
	pgxConn, _ := pgx.Connect(context.TODO(), psqlInfo)
	adapter := pgx4adapter.NewConn(pgxConn)
	tokenStore, _ := pg.NewTokenStore(adapter, pg.WithTokenStoreGCInterval(time.Minute))
	defer tokenStore.Close()

	manager := manage.NewDefaultManager()
	// set default code ttl
	manager.SetAuthorizeCodeTokenCfg(manage.DefaultAuthorizeCodeTokenCfg)
	// token store
	//manager.MustTokenStorage(store.NewMemoryTokenStore())
	manager.MapTokenStorage(tokenStore)
	// generate jwt access token
	// kid is kid in jwt header
	// key is 256-bit-secret
	manager.MapAccessGenerate(generates.NewJWTAccessGenerate("thisiskid", []byte("thisiskey"), jwt.SigningMethodHS512))
	// this is default
	//manager.MapAccessGenerate(generates.NewAccessGenerate())

	//clientStore := store.NewClientStore()
	// clientStore.Set("222222", &models.Client{
	// 	ID:     "222222",
	// 	Secret: "22222222",
	// 	Domain: "http://localhost:8082",
	// })

	clientStore, _ := pg.NewClientStore(adapter)
	client_info := models.Client{
		ID:     "222222",
		Secret: "22222222",
		Domain: "http://localhost:8082",
	}
	clientStore.Create(&client_info)
	manager.MapClientStorage(clientStore)

	srv := server.NewServer(server.NewConfig(), manager)

	srv.SetAllowGetAccessRequest(true)
	// default is ClientBasicHandler
	//srv.SetClientInfoHandler(server.ClientFormHandler)

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
	_ = dumpRequest(os.Stdout, "authorize", r)

	store, err := session.Start(r.Context(), w, r)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	var form url.Values
	if v, ok := store.Get("ReturnUri"); ok {
		form = v.(url.Values)
	}
	r.Form = form

	store.Delete("ReturnUri")
	store.Save()

	err = handler.HandleAuthorizeRequest(w, r)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
	}
}

func (handler *OAuthHandler) TokenHandler(w http.ResponseWriter, r *http.Request) {
	_ = dumpRequest(os.Stdout, "token", r)

	err := handler.HandleTokenRequest(w, r)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
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
	outputHTML(w, r, "public/login.html")
}

func (handler *OAuthHandler) AuthHandler(w http.ResponseWriter, r *http.Request) {
	_ = dumpRequest(os.Stdout, "auth", r)
	store, err := session.Start(context.TODO(), w, r)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	if _, ok := store.Get("LoggedInUserID"); !ok {
		w.Header().Set("Location", "/login")
		w.WriteHeader(http.StatusFound)
		return
	}

	outputHTML(w, r, "public/auth.html")
}

func (handler *OAuthHandler) TestHandler(w http.ResponseWriter, r *http.Request) {
	_ = dumpRequest(os.Stdout, "test", r)
	token, err := handler.ValidationBearerToken(r)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	data := map[string]interface{}{
		"expies_in": int64(time.Until(token.GetAccessCreateAt().Add(token.GetAccessExpiresIn())).Seconds()),
		"client_id": token.GetClientID(),
		"user_id":   token.GetUserID(),
	}
	e := json.NewEncoder(w)
	e.SetIndent("", " ")
	e.Encode(data)
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
