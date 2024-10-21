package main

/*
#cgo LDFLAGS: -L${SRCDIR}/../../target/release -lpolycrypt_rs
#include <stdlib.h>
#include <stdint.h>

typedef struct {
    uint8_t* data;
    uintptr_t len;
} ByteArray;

ByteArray encrypt(const uint8_t* plaintext, uintptr_t plaintext_len, const uint8_t* key);
ByteArray decrypt(const uint8_t* ciphertext, uintptr_t ciphertext_len, const uint8_t* key);
void free_byte_array(ByteArray arr);
void init_logger();
char* decrypt_fields(const char* record, const char* fields_to_decrypt, const uint8_t* key);
char* encrypt_fields(const char* record, const char* fields_to_encrypt, const uint8_t* key);
void free_c_char(char* s);

// Add these new function declarations
char* decrypt_fields_in_batch(const char* records, const char* fields_to_decrypt, const uint8_t* key);
char* encrypt_fields_in_batch(const char* records, const char* fields_to_encrypt, const uint8_t* key);
*/
import "C"
import (
	"encoding/json"
	"fmt"
	"os"
	"unsafe"
)

func encrypt(plaintext []byte, key []byte) ([]byte, error) {
	cPlaintext := (*C.uint8_t)(unsafe.Pointer(&plaintext[0]))
	cKey := (*C.uint8_t)(unsafe.Pointer(&key[0]))

	result := C.encrypt(cPlaintext, C.uintptr_t(len(plaintext)), cKey)
	defer C.free_byte_array(result)

	if result.data == nil {
		return nil, fmt.Errorf("encryption failed")
	}

	return C.GoBytes(unsafe.Pointer(result.data), C.int(result.len)), nil
}

func decrypt(ciphertext []byte, key []byte) ([]byte, error) {
	cCiphertext := (*C.uint8_t)(unsafe.Pointer(&ciphertext[0]))
	cKey := (*C.uint8_t)(unsafe.Pointer(&key[0]))

	result := C.decrypt(cCiphertext, C.uintptr_t(len(ciphertext)), cKey)
	defer C.free_byte_array(result)

	if result.data == nil {
		return nil, fmt.Errorf("decryption failed")
	}

	return C.GoBytes(unsafe.Pointer(result.data), C.int(result.len)), nil
}

func encryptFields(record map[string]interface{}, fieldsToEncrypt []string, key []byte) (map[string]interface{}, error) {
	recordJSON, err := json.Marshal(record)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal record: %v", err)
	}
	fieldsJSON, err := json.Marshal(fieldsToEncrypt)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal fields: %v", err)
	}
	cRecord := C.CString(string(recordJSON))
	cFields := C.CString(string(fieldsJSON))
	cKey := (*C.uint8_t)(unsafe.Pointer(&key[0]))
	defer C.free(unsafe.Pointer(cRecord))
	defer C.free(unsafe.Pointer(cFields))

	cResult := C.encrypt_fields(cRecord, cFields, cKey)
	defer C.free_c_char(cResult)

	if cResult == nil {
		return nil, fmt.Errorf("field encryption failed")
	}

	var result map[string]interface{}
	err = json.Unmarshal([]byte(C.GoString(cResult)), &result)
	if err != nil {
		return nil, fmt.Errorf("failed to unmarshal encrypted result: %v", err)
	}
	return result, nil
}

func decryptFields(record map[string]interface{}, fieldsToDecrypt []string, key []byte) (map[string]interface{}, error) {
	recordJSON, err := json.Marshal(record)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal record: %v", err)
	}
	fieldsJSON, err := json.Marshal(fieldsToDecrypt)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal fields: %v", err)
	}
	cRecord := C.CString(string(recordJSON))
	cFields := C.CString(string(fieldsJSON))
	cKey := (*C.uint8_t)(unsafe.Pointer(&key[0]))
	defer C.free(unsafe.Pointer(cRecord))
	defer C.free(unsafe.Pointer(cFields))

	cResult := C.decrypt_fields(cRecord, cFields, cKey)
	defer C.free_c_char(cResult)

	if cResult == nil {
		return nil, fmt.Errorf("field decryption failed")
	}

	var result map[string]interface{}
	err = json.Unmarshal([]byte(C.GoString(cResult)), &result)
	if err != nil {
		return nil, fmt.Errorf("failed to unmarshal decrypted result: %v", err)
	}

	// Convert []interface{} back to []string for array fields
	for key, value := range result {
		if slice, ok := value.([]interface{}); ok {
			stringSlice := make([]string, len(slice))
			for i, v := range slice {
				stringSlice[i] = fmt.Sprint(v)
			}
			result[key] = stringSlice
		}
	}

	return result, nil
}

func decryptFieldsInBatch(records []map[string]interface{}, fieldsToDecrypt []string, key []byte) ([]map[string]interface{}, error) {
	recordsJSON, err := json.Marshal(records)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal records: %v", err)
	}
	fieldsJSON, err := json.Marshal(fieldsToDecrypt)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal fields: %v", err)
	}
	cRecords := C.CString(string(recordsJSON))
	cFields := C.CString(string(fieldsJSON))
	cKey := (*C.uint8_t)(unsafe.Pointer(&key[0]))
	defer C.free(unsafe.Pointer(cRecords))
	defer C.free(unsafe.Pointer(cFields))

	cResult := C.decrypt_fields_in_batch(cRecords, cFields, cKey)
	defer C.free_c_char(cResult)

	if cResult == nil {
		return nil, fmt.Errorf("batch field decryption failed")
	}

	var result []map[string]interface{}
	err = json.Unmarshal([]byte(C.GoString(cResult)), &result)
	if err != nil {
		return nil, fmt.Errorf("failed to unmarshal decrypted result: %v", err)
	}

	// Convert []interface{} back to []string for array fields
	for _, record := range result {
		for field, value := range record {
			if slice, ok := value.([]interface{}); ok {
				stringSlice := make([]string, len(slice))
				for i, v := range slice {
					stringSlice[i] = fmt.Sprint(v)
				}
				record[field] = stringSlice
			}
		}
	}

	return result, nil
}

func encryptFieldsInBatch(records []map[string]interface{}, fieldsToEncrypt []string, key []byte) ([]map[string]interface{}, error) {
	recordsJSON, err := json.Marshal(records)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal records: %v", err)
	}
	fieldsJSON, err := json.Marshal(fieldsToEncrypt)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal fields: %v", err)
	}
	cRecords := C.CString(string(recordsJSON))
	cFields := C.CString(string(fieldsJSON))
	cKey := (*C.uint8_t)(unsafe.Pointer(&key[0]))
	defer C.free(unsafe.Pointer(cRecords))
	defer C.free(unsafe.Pointer(cFields))

	cResult := C.encrypt_fields_in_batch(cRecords, cFields, cKey)
	defer C.free_c_char(cResult)

	if cResult == nil {
		return nil, fmt.Errorf("batch field encryption failed")
	}

	var result []map[string]interface{}
	err = json.Unmarshal([]byte(C.GoString(cResult)), &result)
	if err != nil {
		return nil, fmt.Errorf("failed to unmarshal encrypted result: %v", err)
	}

	return result, nil
}

func main() {
	// Set the RUST_LOG environment variable
	os.Setenv("RUST_LOG", "info")

	// Initialize the logger
	C.init_logger()

	plaintext := []byte("Hello, world!")
	key := make([]byte, 32)
	// In a real scenario, use a proper key generation method

	encrypted, err := encrypt(plaintext, key)
	if err != nil {
		fmt.Printf("Encryption error: %v\n", err)
		return
	}

	fmt.Printf("Plaintext: %s\n", plaintext)
	fmt.Printf("Encrypted: %v\n", encrypted)
	fmt.Printf("Encrypted length: %d\n", len(encrypted))

	decrypted, err := decrypt(encrypted, key)
	if err != nil {
		fmt.Printf("Decryption error: %v\n", err)
		return
	}

	fmt.Printf("Decrypted: %s\n", decrypted)
	fmt.Printf("Decrypted length: %d\n", len(decrypted))

	// Test encrypt_fields and decrypt_fields
	record := map[string]interface{}{
		"id":             "1234",
		"name":           "John Doe",
		"sensitive_data": "This is sensitive information",
		"array_field":    []string{"item1", "item2", "item3"},
	}
	record2 := map[string]interface{}{
		"id":             "5678",
		"name":           "Jane Smith",
		"sensitive_data": "Another piece of sensitive information",
		"array_field":    []string{"item4", "item5", "item6"},
	}
	records := []map[string]interface{}{record, record2}
	fieldsToEncrypt := []string{"sensitive_data", "array_field"}

	fmt.Printf("Original record: %v\n", record)

	encryptedRecord, err := encryptFields(record, fieldsToEncrypt, key)
	if err != nil {
		fmt.Printf("Field encryption error: %v\n", err)
		return
	}
	fmt.Printf("Encrypted record: %v\n", encryptedRecord)

	decryptedRecord, err := decryptFields(encryptedRecord, fieldsToEncrypt, key)
	if err != nil {
		fmt.Printf("Field decryption error: %v\n", err)
		return
	}
	fmt.Printf("Decrypted record: %v\n", decryptedRecord)

	// Test encrypt_fields_in_batch and decrypt_fields_in_batch
	encryptedRecords, err := encryptFieldsInBatch(records, fieldsToEncrypt, key)
	if err != nil {
		fmt.Printf("Batch field encryption error: %v\n", err)
		return
	}
	fmt.Printf("Encrypted batch records: %v\n", encryptedRecords)

	decryptedRecords, err := decryptFieldsInBatch(encryptedRecords, fieldsToEncrypt, key)
	if err != nil {
		fmt.Printf("Batch field decryption error: %v\n", err)
		return
	}
	fmt.Printf("Decrypted batch records: %v\n", decryptedRecords)
}
