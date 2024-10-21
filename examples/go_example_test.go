package main

import (
	"encoding/json"
	"fmt"
	"reflect"
	"testing"
)

// Keep the existing main() function

func TestEncryptDecrypt(t *testing.T) {
	plaintext := []byte("Hello, world!")
	key := make([]byte, 32)

	encrypted, err := encrypt(plaintext, key)
	if err != nil {
		t.Fatalf("Encryption failed: %v", err)
	}
	if len(encrypted) == 0 {
		t.Fatal("Encryption failed: empty result")
	}

	decrypted, err := decrypt(encrypted, key)
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

	encryptedRecord, err := encryptFields(record, fieldsToEncrypt, key)
	if err != nil {
		t.Fatalf("Field encryption failed: %v", err)
	}

	if reflect.DeepEqual(encryptedRecord["sensitive_data"], record["sensitive_data"]) {
		t.Errorf("Sensitive data was not encrypted")
	}

	decryptedRecord, err := decryptFields(encryptedRecord, fieldsToEncrypt, key)
	if err != nil {
		t.Fatalf("Field decryption failed: %v", err)
	}

	// Detailed comparison
	for k, v := range record {
		if !reflect.DeepEqual(v, decryptedRecord[k]) {
			t.Errorf("Mismatch in field %s:\nOriginal: %#v\nDecrypted: %#v", k, v, decryptedRecord[k])
		}
	}

	// Check for extra fields in decrypted record
	for k := range decryptedRecord {
		if _, exists := record[k]; !exists {
			t.Errorf("Extra field in decrypted record: %s", k)
		}
	}

	// If the above checks pass but DeepEqual still fails, print both records
	if !reflect.DeepEqual(record, decryptedRecord) {
		originalJSON, _ := json.MarshalIndent(record, "", "  ")
		decryptedJSON, _ := json.MarshalIndent(decryptedRecord, "", "  ")
		t.Errorf("Records do not match.\nOriginal:\n%s\nDecrypted:\n%s", originalJSON, decryptedJSON)
	}
}

// Add this helper function to print maps for debugging
func printMap(m map[string]interface{}) string {
	result := "{\n"
	for k, v := range m {
		result += fmt.Sprintf("  %s: %#v\n", k, v)
	}
	result += "}"
	return result
}
