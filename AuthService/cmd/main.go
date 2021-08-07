package main

import (
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"strconv"

	oauth "github.com/futugyou/AnimalDeCompagnieNoSu/AuthService/pkg/oauth"
	cron "github.com/robfig/cron/v3"
)

const clientID = "clientID"
const clientSecret = "clientSecret"

func main() {
	// fs := http.FileServer(http.Dir("public"))
	// http.Handle("/", fs)
	// http.HandleFunc("/oauth/redirect", authRedirectHandleFunc)
	srv := oauth.New()
	http.HandleFunc("/authorize", srv.AuthorizeHandler)
	http.HandleFunc("/token", srv.TokenHandler)
	http.HandleFunc("/login", srv.LoginHandler)
	http.HandleFunc("/auth", srv.AuthHandler)
	http.HandleFunc("/test", srv.TestHandler)
	http.ListenAndServe(":8080", nil)

	// unuse code now
	for i := 0; i < 2; i++ {
		go func(i int) {
			crontab := cron.New()
			task := func() {
				fmt.Println("Hello:" + strconv.Itoa(i))
			}
			crontab.AddFunc("* * * * *", task)
			crontab.Start()
		}(i)
	}

	select {}
	//println("Hello!")
}

func authRedirectHandleFunc(w http.ResponseWriter, r *http.Request) {
	err := r.ParseForm()
	if err != nil {
		fmt.Fprintf(os.Stdout, "could not parse query: %v", err)
		w.WriteHeader(http.StatusBadRequest)
	}
	code := r.FormValue("code")

	reurl := fmt.Sprintf("https://github.com/login/oauth/access_token?client_id=%s&client_secret=%s&code=%s", clientID, clientSecret, code)
	req, err := http.NewRequest(http.MethodPost, reurl, nil)

	if err != nil {
		fmt.Fprintf(os.Stdout, "could not create HTTP request: %v", err)
		w.WriteHeader(http.StatusBadRequest)
	}
	req.Header.Set("accept", "application/json")

	httpClient := http.Client{}
	res, err := httpClient.Do(req)
	if err != nil {
		fmt.Fprintf(os.Stdout, "could not send HTTP request: %v", err)
		w.WriteHeader(http.StatusInternalServerError)
	}
	defer res.Body.Close()

	var t OAuthAccessResponse
	if err := json.NewDecoder(res.Body).Decode(&t); err != nil {
		fmt.Fprintf(os.Stdout, "could not parse JSON response: %v", err)
		w.WriteHeader(http.StatusBadRequest)
	}
	w.Header().Set("Location", "/welcome.html?access_token="+t.AccessToken)
	w.WriteHeader(http.StatusFound)
}

type OAuthAccessResponse struct {
	AccessToken string `json:"access_token"`
}
