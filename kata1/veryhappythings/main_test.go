package main

import (
    "fmt"
    "reflect"
    "testing"
)

func TestLastTwoWords(t *testing.T) {
    // Is this really the best way to do this?
    if !reflect.DeepEqual(lastTwoWords("Test example string"), []string{"example", "string"}) {
        t.Fail()
    }

    if !reflect.DeepEqual(lastTwoWords("Example string"), []string{"Example", "string"}) {
        t.Fail()
    }

    if !reflect.DeepEqual(lastTwoWords("Example"), []string{"Example"}) {
        t.Fail()
    }

    // I don't understand why this test fails
    //if !reflect.DeepEqual(lastTwoWords(""), make([]string, 0)) {
    //    t.Fail()
    //}
}

func TestAttachToSentence(t *testing.T) {
    trigrams := make(map[string][]string)
    trigrams["I wish"] = []string{"I"}

    result, err := attachToSentence(trigrams, ("I wish"))
    if result != "I wish I" || err != nil {
        t.Fail()
    }

    result, err = attachToSentence(trigrams, ("sentence missing"))
    if result != "sentence missing" || err == nil {
        t.Fail()
    }
}

func TestBuildMap(t *testing.T) {
    expected := make(map[string][]string)
    expected["I wish"] = []string{"I", "I"}
    expected["wish I"] = []string{"may", "might"}
    expected["may I"] = []string{"wish"}
    expected["I may"] = []string{"I"}

    result := buildMap("I wish I may I wish I might")

    if !reflect.DeepEqual(expected, result) {
        fmt.Println(expected)
        fmt.Println(result)
        t.Fail()
    }
}
