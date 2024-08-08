#include <Windows.h>
#include <TlHelp32.h>
#include <iostream>
#include <string>

extern "C" __declspec(dllexport) DWORD find_pid_by_name(const wchar_t* process_name) {
    DWORD pid = 0;
    HANDLE hSnap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
    if (hSnap != INVALID_HANDLE_VALUE) {
        PROCESSENTRY32W pe;
        pe.dwSize = sizeof(PROCESSENTRY32W);

        if (Process32FirstW(hSnap, &pe)) {
            do {
                std::wstring pe_szExeFile = pe.szExeFile;

                if (pe_szExeFile == process_name) {
                    pid = pe.th32ProcessID;
                    break;
                }
            } while (Process32NextW(hSnap, &pe));
        }
    }
    return pid;
}

extern "C" __declspec(dllexport) bool get_process_path(DWORD pid, wchar_t* path, DWORD size) {
    HANDLE hProcess = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, pid);
    if (hProcess) {
        if (QueryFullProcessImageNameW(hProcess, 0, path, &size)) {
            CloseHandle(hProcess);
            return true;
        }
        CloseHandle(hProcess);
    }
    return false;
}

extern "C" __declspec(dllexport) bool terminate_process(DWORD pid) {
    HANDLE hProcess = OpenProcess(PROCESS_TERMINATE | SYNCHRONIZE, FALSE, pid);
    if (hProcess == NULL) {
        return false;
    }

    if (!TerminateProcess(hProcess, 1)) {
        CloseHandle(hProcess);
        return false;
    }

    WaitForSingleObject(hProcess, INFINITE);

    HANDLE hSnapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
    if (hSnapshot != INVALID_HANDLE_VALUE) {
        PROCESSENTRY32W pe;
        pe.dwSize = sizeof(PROCESSENTRY32W);

        if (Process32FirstW(hSnapshot, &pe)) {
            do {
                if (pe.th32ParentProcessID == pid) {
                    HANDLE hChildProcess = OpenProcess(PROCESS_TERMINATE | SYNCHRONIZE, FALSE, pe.th32ProcessID);
                    if (hChildProcess != NULL) {
                        TerminateProcess(hChildProcess, 1);
                        WaitForSingleObject(hChildProcess, INFINITE);
                        CloseHandle(hChildProcess);
                    }
                }
            } while (Process32NextW(hSnapshot, &pe));
        }
        CloseHandle(hSnapshot);
    }

    CloseHandle(hProcess);
    return true;
}
extern "C" __declspec(dllexport) bool start_process_detached(const wchar_t* exe_path) {
    STARTUPINFOW si = {0};
    PROCESS_INFORMATION pi = {0};

    si.cb = sizeof(si); 

    BOOL result = CreateProcessW(
        exe_path,     
        nullptr,       
        nullptr,        
        nullptr,        
        FALSE,       
        CREATE_NEW_CONSOLE | DETACHED_PROCESS, 
        nullptr,       
        nullptr,      
        &si,          
        &pi             
    );

    if (result) {
        CloseHandle(pi.hProcess);
        CloseHandle(pi.hThread);
        return true;
    }

    return false;
}