package main

import (
	"fmt"
	"reflect"
	"testing"

	"go-ffi-test/polycrypt"
)

func TestEncryptDecrypt(t *testing.T) {
	plaintext := []byte("Hello, world!")
	key := make([]byte, 32)
	// polycrypt.InitLogger()
	pc := polycrypt.NewPolyCrypt(key)

	encrypted, err := pc.Encrypt(plaintext)
	if err != nil {
		t.Fatalf("Encryption failed: %v", err)
	}
	if len(encrypted) == 0 {
		t.Fatal("Encryption failed: empty result")
	}

	decrypted, err := pc.Decrypt(encrypted)
	if err != nil {
		t.Fatalf("Decryption failed: %v", err)
	}

	if !reflect.DeepEqual(plaintext, decrypted) {
		t.Errorf("Decrypted text does not match original plaintext")
	}
}

func TestFieldEncryptionDecryption(t *testing.T) {
	record := map[string]interface{}{
		"id":             "1234",
		"name":           "John Doe",
		"sensitive_data": "This is sensitive information",
		"array_field":    []string{"item1", "item2", "item3"},
	}
	fieldsToEncrypt := []string{"sensitive_data", "array_field"}
	key := make([]byte, 32)
	pc := polycrypt.NewPolyCrypt(key)

	encryptedRecord, err := pc.EncryptFields(record, fieldsToEncrypt)
	if err != nil {
		t.Fatalf("Field encryption failed: %v", err)
	}

	decryptedRecord, err := pc.DecryptFields(encryptedRecord, fieldsToEncrypt)
	if err != nil {
		t.Fatalf("Field decryption failed: %v", err)
	}

	if !reflect.DeepEqual(record, decryptedRecord) {
		t.Errorf("Decrypted record does not match original record.\nOriginal: %+v\nDecrypted: %+v", record, decryptedRecord)
	}
}

func TestBatchFieldEncryptionDecryption(t *testing.T) {
	records := []map[string]interface{}{
		{
			"id":             "1234",
			"name":           "John Doe",
			"sensitive_data": "This is sensitive information",
			"array_field":    []string{"item1", "item2", "item3"},
		},
		{
			"id":             "5678",
			"name":           "Jane Smith",
			"sensitive_data": "Another piece of sensitive information",
			"array_field":    []string{"item4", "item5", "item6"},
		},
	}
	fieldsToEncrypt := []string{"sensitive_data", "array_field"}
	key := make([]byte, 32)
	pc := polycrypt.NewPolyCrypt(key)

	encryptedRecords, err := pc.EncryptFieldsInBatch(records, fieldsToEncrypt)
	if err != nil {
		t.Fatalf("Batch field encryption failed: %v", err)
	}

	decryptedRecords, err := pc.DecryptFieldsInBatch(encryptedRecords, fieldsToEncrypt)
	if err != nil {
		t.Fatalf("Batch field decryption failed: %v", err)
	}

	if !reflect.DeepEqual(records, decryptedRecords) {
		t.Errorf("Decrypted records do not match original records.\nOriginal: %+v\nDecrypted: %+v", records, decryptedRecords)
	}
}

// Add these helper functions at the end of the file
func printMap(m map[string]interface{}) string {
	result := "{\n"
	for k, v := range m {
		result += fmt.Sprintf("  %s: %#v\n", k, v)
	}
	result += "}"
	return result
}

func printRecords(records []map[string]interface{}) string {
	result := "[\n"
	for i, record := range records {
		result += fmt.Sprintf("  Record %d: %s\n", i, printMap(record))
	}
	result += "]"
	return result
}
