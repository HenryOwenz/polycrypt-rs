package polycrypt

/*
#cgo LDFLAGS: -L${SRCDIR} -lpolycrypt_rs
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
char* decrypt_fields_in_batch(const char* records, const char* fields_to_decrypt, const uint8_t* key);
char* encrypt_fields_in_batch(const char* records, const char* fields_to_encrypt, const uint8_t* key);
void free_c_char(char* s);
*/
import "C"
import (
	"encoding/json"
	"fmt"
	"reflect"
	"unsafe"
)

type PolyCrypt struct {
	Key []byte
}

func NewPolyCrypt(key []byte) *PolyCrypt {
	if len(key) != 32 {
		panic("Key must be 32 bytes long")
	}
	return &PolyCrypt{Key: key}
}

func (pc *PolyCrypt) Encrypt(plaintext []byte) ([]byte, error) {
	if len(plaintext) == 0 {
		return nil, fmt.Errorf("plaintext is empty")
	}
	if len(pc.Key) == 0 {
		return nil, fmt.Errorf("encryption key is empty")
	}
	cPlaintext := (*C.uint8_t)(unsafe.Pointer(&plaintext[0]))
	cKey := (*C.uint8_t)(unsafe.Pointer(&pc.Key[0]))

	result := C.encrypt(cPlaintext, C.uintptr_t(len(plaintext)), cKey)
	defer C.free_byte_array(result)

	if result.data == nil {
		return nil, fmt.Errorf("encryption failed")
	}

	return C.GoBytes(unsafe.Pointer(result.data), C.int(result.len)), nil
}

func (pc *PolyCrypt) Decrypt(ciphertext []byte) ([]byte, error) {
	if len(ciphertext) == 0 {
		return nil, fmt.Errorf("ciphertext is empty")
	}
	if len(pc.Key) == 0 {
		return nil, fmt.Errorf("decryption key is empty")
	}
	cCiphertext := (*C.uint8_t)(unsafe.Pointer(&ciphertext[0]))
	cKey := (*C.uint8_t)(unsafe.Pointer(&pc.Key[0]))

	result := C.decrypt(cCiphertext, C.uintptr_t(len(ciphertext)), cKey)
	defer C.free_byte_array(result)

	if result.data == nil {
		return nil, fmt.Errorf("decryption failed")
	}

	return C.GoBytes(unsafe.Pointer(result.data), C.int(result.len)), nil
}

func (pc *PolyCrypt) EncryptFields(record map[string]interface{}, fieldsToEncrypt []string) (map[string]interface{}, error) {
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
	cKey := (*C.uint8_t)(unsafe.Pointer(&pc.Key[0]))
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

func (pc *PolyCrypt) DecryptFields(record map[string]interface{}, fieldsToDecrypt []string) (map[string]interface{}, error) {
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
	cKey := (*C.uint8_t)(unsafe.Pointer(&pc.Key[0]))
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

func (pc *PolyCrypt) EncryptFieldsInBatch(records []map[string]interface{}, fieldsToEncrypt []string) ([]map[string]interface{}, error) {
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
	cKey := (*C.uint8_t)(unsafe.Pointer(&pc.Key[0]))
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

func (pc *PolyCrypt) DecryptFieldsInBatch(records []map[string]interface{}, fieldsToDecrypt []string) ([]map[string]interface{}, error) {
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
	cKey := (*C.uint8_t)(unsafe.Pointer(&pc.Key[0]))
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

func InitLogger() {
	C.init_logger()
}

// EncryptStruct encrypts specified fields in a struct
func (pc *PolyCrypt) EncryptStruct(v interface{}) error {
	val := reflect.ValueOf(v)
	if val.Kind() != reflect.Ptr || val.Elem().Kind() != reflect.Struct {
		return fmt.Errorf("input must be a pointer to a struct")
	}
	val = val.Elem()
	typ := val.Type()

	fieldsToEncrypt := []string{}
	record := make(map[string]interface{})

	for i := 0; i < val.NumField(); i++ {
		field := val.Field(i)
		fieldType := typ.Field(i)
		tag := fieldType.Tag.Get("encrypt")
		if tag == "true" && field.Kind() == reflect.String {
			fieldsToEncrypt = append(fieldsToEncrypt, fieldType.Name)
			record[fieldType.Name] = field.String()
		}
	}

	recordJSON, err := json.Marshal(record)
	if err != nil {
		return fmt.Errorf("failed to marshal record: %v", err)
	}
	fieldsJSON, err := json.Marshal(fieldsToEncrypt)
	if err != nil {
		return fmt.Errorf("failed to marshal fields: %v", err)
	}

	cRecord := C.CString(string(recordJSON))
	cFields := C.CString(string(fieldsJSON))
	cKey := (*C.uint8_t)(unsafe.Pointer(&pc.Key[0]))
	defer C.free(unsafe.Pointer(cRecord))
	defer C.free(unsafe.Pointer(cFields))

	cResult := C.encrypt_fields(cRecord, cFields, cKey)
	defer C.free_c_char(cResult)

	if cResult == nil {
		return fmt.Errorf("field encryption failed")
	}

	var result map[string]interface{}
	err = json.Unmarshal([]byte(C.GoString(cResult)), &result)
	if err != nil {
		return fmt.Errorf("failed to unmarshal encrypted result: %v", err)
	}

	for i := 0; i < val.NumField(); i++ {
		field := val.Field(i)
		fieldType := typ.Field(i)
		tag := fieldType.Tag.Get("encrypt")
		if tag == "true" && field.Kind() == reflect.String {
			if encryptedValue, ok := result[fieldType.Name]; ok {
				field.SetString(encryptedValue.(string))
			}
		}
	}

	return nil
}

// DecryptStruct decrypts specified fields in a struct
func (pc *PolyCrypt) DecryptStruct(v interface{}) error {
	val := reflect.ValueOf(v)
	if val.Kind() != reflect.Ptr || val.Elem().Kind() != reflect.Struct {
		return fmt.Errorf("input must be a pointer to a struct")
	}
	val = val.Elem()
	typ := val.Type()

	fieldsToDecrypt := []string{}
	record := make(map[string]interface{})

	for i := 0; i < val.NumField(); i++ {
		field := val.Field(i)
		fieldType := typ.Field(i)
		tag := fieldType.Tag.Get("encrypt")
		if tag == "true" && field.Kind() == reflect.String {
			fieldsToDecrypt = append(fieldsToDecrypt, fieldType.Name)
			record[fieldType.Name] = field.String()
		}
	}

	recordJSON, err := json.Marshal(record)
	if err != nil {
		return fmt.Errorf("failed to marshal record: %v", err)
	}
	fieldsJSON, err := json.Marshal(fieldsToDecrypt)
	if err != nil {
		return fmt.Errorf("failed to marshal fields: %v", err)
	}

	cRecord := C.CString(string(recordJSON))
	cFields := C.CString(string(fieldsJSON))
	cKey := (*C.uint8_t)(unsafe.Pointer(&pc.Key[0]))
	defer C.free(unsafe.Pointer(cRecord))
	defer C.free(unsafe.Pointer(cFields))

	cResult := C.decrypt_fields(cRecord, cFields, cKey)
	defer C.free_c_char(cResult)

	if cResult == nil {
		return fmt.Errorf("field decryption failed")
	}

	var result map[string]interface{}
	err = json.Unmarshal([]byte(C.GoString(cResult)), &result)
	if err != nil {
		return fmt.Errorf("failed to unmarshal decrypted result: %v", err)
	}

	for i := 0; i < val.NumField(); i++ {
		field := val.Field(i)
		fieldType := typ.Field(i)
		tag := fieldType.Tag.Get("encrypt")
		if tag == "true" && field.Kind() == reflect.String {
			if decryptedValue, ok := result[fieldType.Name]; ok {
				field.SetString(decryptedValue.(string))
			}
		}
	}

	return nil
}