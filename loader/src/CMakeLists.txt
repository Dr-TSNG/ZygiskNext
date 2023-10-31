cmake_minimum_required(VERSION 3.22.1)
project("loader")

find_package(cxx REQUIRED CONFIG)

aux_source_directory(common COMMON_SRC_LIST)
add_library(common STATIC ${COMMON_SRC_LIST})
target_include_directories(common PRIVATE include)
target_link_libraries(common cxx::cxx log)

aux_source_directory(injector INJECTOR_SRC_LIST)
add_library(zygisk SHARED ${INJECTOR_SRC_LIST})
target_include_directories(zygisk PRIVATE include)
target_link_libraries(zygisk cxx::cxx log common lsplt_static phmap)

aux_source_directory(ptracer PTRACER_SRC_LIST)
add_executable(libptracer.so ${PTRACER_SRC_LIST})
target_include_directories(libptracer.so PRIVATE include)
target_link_libraries(libptracer.so cxx::cxx log common)

add_subdirectory(external)
