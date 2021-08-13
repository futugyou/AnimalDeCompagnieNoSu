package utils

import (
	"fmt"
	"reflect"
	"sync"

	"github.com/petersunbag/coven"
)

var (
	mutex sync.Mutex
	c_map = make(map[string]*coven.Converter)
)

func Map(src, dst interface{}) (err error) {
	key := fmt.Sprintf("%v_%v", reflect.TypeOf(src).String(), reflect.TypeOf(dst).String())
	if _, ok := c_map[key]; !ok {
		mutex.Lock()
		defer mutex.Unlock()
		if c_map[key], err = coven.NewConverter(dst, src); err != nil {
			return
		}
	}
	if err = c_map[key].Convert(dst, src); err != nil {
		return
	}
	return
}
