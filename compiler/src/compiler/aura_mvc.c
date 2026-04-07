#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif

#define _CRT_SECURE_NO_WARNINGS

#ifdef _WIN32
    #include <WinSock2.h>
    #include <Windows.h>
    #include <WS2tcpip.h>
    #include <stdio.h>
    #include <stdlib.h>
    #include <string.h>
#else
    #include <sys/socket.h>
    #include <netinet/in.h>
    #include <unistd.h>
    #include <stdio.h>
    #include <stdlib.h>
    #include <string.h>
#endif

typedef char* (*AuraControllerFunc)(void* instance, long long param);

typedef struct {
    char path[64];
    AuraControllerFunc func;
} Route;

static Route registry[32];
static int route_count = 0;

void aura_mvc_register(const char* path, AuraControllerFunc func_ptr) {
    if (route_count < 32) {
        // Safe copy instead of strncpy
        size_t i = 0;
        for (i = 0; i < 63 && path[i] != '\0'; i++) {
            registry[route_count].path[i] = path[i];
        }
        registry[route_count].path[i] = '\0';
        registry[route_count].func = func_ptr;
        route_count++;
    }
}

// --- AuraView Engine: Template Processor ---

// Simple string replace (MVC version of the Aura Runtime)
static char* mvc_str_replace(const char* orig, const char* rep, const char* with) {
    if (!orig || !rep || !with) return (char*)orig;
    char* result;
    char* ins;
    char* tmp;
    int len_rep = strlen(rep);
    int len_with = strlen(with);
    int count;

    ins = (char*)orig;
    for (count = 0; (tmp = strstr(ins, rep)); ++count) {
        ins = tmp + len_rep;
    }

    tmp = result = malloc(strlen(orig) + (len_with - len_rep) * count + 1);
    if (!result) return NULL;

    while (count--) {
        ins = strstr(orig, rep);
        int len_front = ins - orig;
        tmp = strncpy(tmp, orig, len_front) + len_front;
        tmp = strcpy(tmp, with) + len_with;
        orig += len_front + len_rep;
    }
    strcpy(tmp, orig);
    return result;
}

// Renders a list (array of structs)
// snippet: HTML part inside the foreach
// data: Aura Array pointer (i64 array)
// len: Array length
// field_names: Class field names (comma separated)
char* aura_mvc_render_list(const char* snippet, long long* data, int len, const char* field_names) {
    char* final_res = malloc(1);
    final_res[0] = '\0';
    int total_len = 0;

    char* fields[16];
    int f_count = 0;
    char* f_copy = _strdup(field_names);
    char* token = strtok(f_copy, ",");
    while (token && f_count < 16) {
        fields[f_count++] = token;
        token = strtok(NULL, ",");
    }

    for (int i = 0; i < len; i++) {
        // Each item is essentially a struct pointer (or packed as i64)
        long long item_ptr = data[i];
        char* item_html = _strdup(snippet);

        for (int f = 0; f < f_count; f++) {
            char tag[64];
            sprintf(tag, "{model.%s}", fields[f]);
            
            // In Aura 64-bit, every field is 8-byte (i64)
            long long* field_ptr = (long long*)(item_ptr + (f * 8));
            long long val = *field_ptr;

            char val_str[64];
            if (val > 0x10000) { // Probability of being a string pointer
                char* replaced = mvc_str_replace(item_html, tag, (char*)val);
                free(item_html);
                item_html = replaced;
            } else {
                sprintf(val_str, "%lld", val);
                char* replaced = mvc_str_replace(item_html, tag, val_str);
                free(item_html);
                item_html = replaced;
            }
        }

        total_len += strlen(item_html);
        final_res = realloc(final_res, total_len + 1);
        strcat(final_res, item_html);
        free(item_html);
    }

    free(f_copy);
    return final_res;
}

void aura_mvc_serve(long long sock_ll, void* instance) {
    if (sock_ll == -1) return;

    printf("MVC Server listening...\n");
    while (1) {
#ifdef _WIN32
        long long client_sock = (long long)accept((SOCKET)sock_ll, NULL, NULL);
#else
        long long client_sock = (long long)accept((int)sock_ll, NULL, NULL);
#endif
        if (client_sock == -1) continue;

        char buffer[1024] = {0};
#ifdef _WIN32
        recv((SOCKET)client_sock, buffer, 1024, 0);
#else
        recv((int)client_sock, buffer, 1024, 0);
#endif

        // Basic routing
        char* path_start = strstr(buffer, "GET /");
        if (path_start) {
            path_start += 5;
            char* path_end = strpbrk(path_start, " \r\n");
            if (path_end) {
                char method_name[64] = {0};
                size_t len = path_end - path_start;
                char* q_mark = strchr(path_start, '?');
                if (q_mark && q_mark < path_end) {
                    len = q_mark - path_start;
                }
                if (len > 63) len = 63;
                
                // Safe copy
                for (size_t i = 0; i < len; i++) {
                    method_name[i] = path_start[i];
                }
                method_name[len] = '\0';

                // Search in Registry
                int found = 0;
                for (int i = 0; i < route_count; i++) {
                    if (strcmp(registry[i].path, method_name) == 0) {
                        // Parameter parsing (simply extracting characters after = following ?)
                        long long param = 0;
                        char* q = strchr(path_start, '?');
                        if (q && q < path_end) {
                            char* eq = strchr(q, '=');
                            if (eq && eq < path_end) {
                                sscanf(eq + 1, "%lld", &param);
                            }
                        }

                        char* response_body = registry[i].func(instance, param);
                        
                        char header[256];
                        int h_len = sprintf(header, "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: %zu\r\n\r\n", strlen(response_body));
                        
#ifdef _WIN32
                        send((SOCKET)client_sock, header, h_len, 0);
                        send((SOCKET)client_sock, response_body, (int)strlen(response_body), 0);
#else
                        send((int)client_sock, header, h_len, 0);
                        send((int)client_sock, response_body, (int)strlen(response_body), 0);
#endif
                        found = 1;
                        break;
                    }
                }

                if (!found) {
                    char error404[] = "HTTP/1.1 404 Not Found\r\n\r\n{\"error\":\"Route not found\"}";
#ifdef _WIN32
                    send((SOCKET)client_sock, error404, (int)strlen(error404), 0);
#else
                    send((int)client_sock, error404, (int)strlen(error404), 0);
#endif
                }
            }
        }

#ifdef _WIN32
        closesocket((SOCKET)client_sock);
#else
        close((int)client_sock);
#endif
    }
}
