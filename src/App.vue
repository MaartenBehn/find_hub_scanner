<template>
  <div class="min-h-screen bg-white dark:bg-gray-900 transition-colors duration-200">
    <!-- Header -->
    <header class="sticky pt-4 sm:p-0 top-0 z-50 border-b border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-950 shadow-sm">
      <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-3 sm:py-4">
        <div class="flex items-center justify-between gap-2 sm:gap-4">
          <div class="flex items-center gap-2 sm:gap-3 flex-1 min-w-0">
            <Bluetooth class="w-6 h-6 sm:w-8 sm:h-8 text-blue-600 dark:text-blue-400 shrink-0" />
            <h1 class="text-xl sm:text-2xl md:text-3xl font-bold text-gray-900 dark:text-white truncate">
              Tauri Ble Example
            </h1>
          </div>
          <div class="flex items-center gap-1 sm:gap-2">
            <button
              @click="checkPermissions"
              class="px-3 sm:px-4 py-2 rounded-lg bg-blue-600 hover:bg-blue-700 dark:bg-blue-700 dark:hover:bg-blue-600 text-white text-sm font-medium transition-colors"
              :disabled="isLoading"
            >
              <Check class="w-4 h-4 sm:w-5 sm:h-5" />
            </button>
            <button
              @click="toggleScan"
              :class="[
                'px-3 sm:px-4 py-2 rounded-lg text-white text-sm font-medium transition-colors flex items-center gap-1 sm:gap-2',
                isScanning
                  ? 'bg-red-600 hover:bg-red-700 dark:bg-red-700 dark:hover:bg-red-600'
                  : 'bg-green-600 hover:bg-green-700 dark:bg-green-700 dark:hover:bg-green-600'
              ]"
              :disabled="isLoading || adapterState !== 'PoweredOn'"
            >
              <Wifi class="w-4 h-4 sm:w-5 sm:h-5" />
              <span class="hidden sm:inline">{{ isScanning ? 'Stop' : 'Scan' }}</span>
            </button>
          </div>
        </div>
      </div>
    </header>

    <!-- Main Content -->
    <main class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-4 sm:py-6">
      <!-- Bluetooth Off Alert -->
      <div v-if="adapterState !== 'PoweredOn' && adapterState !== 'Unknown'" class="mb-4 sm:mb-6 p-4 rounded-lg bg-red-50 dark:bg-red-900/30 border border-red-200 dark:border-red-800 flex items-start gap-3">
        <AlertCircle class="w-5 h-5 sm:w-6 sm:h-6 text-red-600 dark:text-red-400 mt-0.5 shrink-0" />
        <div class="flex-1 min-w-0">
          <h3 class="font-semibold text-red-800 dark:text-red-300 text-sm sm:text-base mb-1">Bluetooth is Off</h3>
          <p class="text-red-700 dark:text-red-200 text-xs sm:text-sm">
            Please enable Bluetooth on your device. Adapter State: <span class="font-mono">{{ adapterState }}</span>
          </p>
        </div>
      </div>

      <!-- Status Card -->
      <div class="bg-gray-50 dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-4 mb-4 sm:mb-6">
        <div class="grid grid-cols-2 md:grid-cols-4 gap-3 sm:gap-4">
          <div class="flex flex-col">
            <span class="text-xs sm:text-sm text-gray-600 dark:text-gray-400">Adapter State</span>
            <span class="text-sm sm:text-base font-semibold text-gray-900 dark:text-white flex items-center gap-1 mt-1">
              <Activity :class="['w-4 h-4', adapterState === 'PoweredOn' ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400']" />
              {{ adapterState }}
            </span>
          </div>
          <div class="flex flex-col">
            <span class="text-xs sm:text-sm text-gray-600 dark:text-gray-400">Scanning</span>
            <span class="text-sm sm:text-base font-semibold text-gray-900 dark:text-white flex items-center gap-1 mt-1">
              <Activity :class="['w-4 h-4', isScanning ? 'text-green-600 dark:text-green-400' : 'text-gray-400']" />
              {{ isScanning ? 'Yes' : 'No' }}
            </span>
          </div>
          <div class="flex flex-col">
            <span class="text-xs sm:text-sm text-gray-600 dark:text-gray-400">Connected</span>
            <span class="text-sm sm:text-base font-semibold text-gray-900 dark:text-white flex items-center gap-1 mt-1">
              <Link2 :class="['w-4 h-4', connectedDevice ? 'text-green-600 dark:text-green-400' : 'text-gray-400']" />
              {{ connectedDevice ? 'Yes' : 'No' }}
            </span>
          </div>
          <div class="flex flex-col">
            <span class="text-xs sm:text-sm text-gray-600 dark:text-gray-400">Devices Found</span>
            <span class="text-sm sm:text-base font-semibold text-gray-900 dark:text-white">{{ devices.size }}</span>
          </div>
        </div>
      </div>

      <!-- Tab Navigation -->
      <div class="flex gap-2 sm:gap-4 mb-4 sm:mb-6 border-b border-gray-200 dark:border-gray-700 overflow-x-auto">
        <button
          v-for="tab in tabs"
          :key="tab"
          @click="activeTab = tab"
          :class="[
            'px-3 sm:px-4 py-2 sm:py-3 font-medium text-sm sm:text-base whitespace-nowrap border-b-2 transition-colors',
            activeTab === tab
              ? 'border-blue-600 dark:border-blue-400 text-blue-600 dark:text-blue-400'
              : 'border-transparent text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-300'
          ]"
        >
          {{ tab }}
        </button>
      </div>

      <!-- Tab Content -->
      <div class="space-y-4">
        <!-- Connection Error Alert -->
        <div v-if="connectionErrorVisible && connectionError" class="p-4 rounded-lg bg-red-50 dark:bg-red-900/30 border border-red-200 dark:border-red-800 flex items-start gap-3 animate-pulse">
          <AlertCircle class="w-5 h-5 sm:w-6 sm:h-6 text-red-600 dark:text-red-400 mt-0.5 shrink-0" />
          <div class="flex-1 min-w-0">
            <h3 class="font-semibold text-red-800 dark:text-red-300 text-sm sm:text-base mb-1">Connection Error</h3>
            <p class="text-red-700 dark:text-red-200 text-xs sm:text-sm">
              {{ connectionError }}
            </p>
          </div>
          <button
            @click="connectionErrorVisible = false"
            class="text-red-600 dark:text-red-400 hover:text-red-800 dark:hover:text-red-200 shrink-0 font-bold text-lg leading-none"
          >
            ×
          </button>
        </div>

        <!-- Devices Tab -->
        <div v-if="activeTab === 'Devices'" class="space-y-3">
          <!-- Search Box -->
          <div class="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-3 sm:p-4">
            <input
              v-model="searchQuery"
              type="text"
              placeholder="Search device name or address..."
              class="w-full px-3 sm:px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-sm placeholder-gray-500 dark:placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-600 dark:focus:ring-blue-400"
            />
            <div v-if="searchQuery" class="mt-2 text-xs text-gray-600 dark:text-gray-400">
              Search Results: {{ filteredDevices.length }} / {{ devices.size }} devices
            </div>
          </div>

          <div v-if="isLoading" class="flex items-center justify-center py-8">
            <Loader class="w-6 h-6 sm:w-8 sm:h-8 text-blue-600 dark:text-blue-400 animate-spin" />
          </div>
          <div v-else-if="devices.size === 0" class="text-center py-8">
            <Bluetooth class="w-12 h-12 sm:w-16 sm:h-16 text-gray-400 dark:text-gray-600 mx-auto mb-3 opacity-50" />
            <p class="text-gray-600 dark:text-gray-400 text-sm sm:text-base">
              {{ isScanning ? 'Scanning for devices...' : 'Start scanning to find devices' }}
            </p>
          </div>
          <div v-else-if="filteredDevices.length === 0" class="text-center py-8">
            <Bluetooth class="w-12 h-12 sm:w-16 sm:h-16 text-gray-400 dark:text-gray-600 mx-auto mb-3 opacity-50" />
            <p class="text-gray-600 dark:text-gray-400 text-sm sm:text-base">
              No matching devices found
            </p>
          </div>
          <div v-else class="grid gap-3">
            <div
              v-for="device of filteredDevices"
              :key="device.address"
              class="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-3 sm:p-4 hover:border-blue-400 dark:hover:border-blue-500 transition-colors"
            >
              <div class="flex items-start justify-between gap-3 mb-2">
                <div class="flex-1 min-w-0">
                  <h3 class="font-semibold text-gray-900 dark:text-white truncate text-sm sm:text-base">
                    {{ device.name || 'Unknown Device' }}
                  </h3>
                  <p class="text-xs sm:text-sm text-gray-600 dark:text-gray-400 font-mono truncate">
                    {{ device.address }}
                  </p>
                </div>
                <div class="text-right shrink-0">
                  <div class="inline-flex items-center gap-1 bg-blue-100 dark:bg-blue-900 px-2 py-1 rounded text-xs sm:text-sm font-medium text-blue-900 dark:text-blue-100">
                    <Signal :class="['w-3 h-3 sm:w-4 sm:h-4']" />
                    {{ device.rssi ?? 'N/A' }}
                  </div>
                </div>
              </div>
              <button
                @click="connectToDevice(device)"
                :disabled="isLoading || (connectedDevice?.address === device.address)"
                :class="[
                  'w-full px-3 sm:px-4 py-2 rounded-lg font-medium text-sm transition-colors',
                  connectedDevice?.address === device.address
                    ? 'bg-green-100 dark:bg-green-900 text-green-900 dark:text-green-100 cursor-not-allowed'
                    : 'bg-blue-600 hover:bg-blue-700 dark:bg-blue-700 dark:hover:bg-blue-600 text-white'
                ]"
              >
                {{ connectedDevice?.address === device.address ? 'Connected' : 'Connect' }}
              </button>
            </div>
          </div>
        </div>

        <!-- Services Tab -->
        <div v-if="activeTab === 'Services'" class="space-y-3">
          <div v-if="!connectedDevice" class="text-center py-8 text-gray-600 dark:text-gray-400">
            <Link2Off class="w-12 h-12 sm:w-16 sm:h-16 text-gray-400 dark:text-gray-600 mx-auto mb-3 opacity-50" />
            <p class="text-sm sm:text-base">Connect to a device to view services</p>
          </div>
          <div v-else-if="isLoadingServices" class="flex items-center justify-center py-8">
            <Loader class="w-6 h-6 sm:w-8 sm:h-8 text-blue-600 dark:text-blue-400 animate-spin" />
          </div>
          <div v-else-if="services.length === 0" class="text-center py-8 text-gray-600 dark:text-gray-400">
            <AlertCircle class="w-12 h-12 sm:w-16 sm:h-16 text-gray-400 dark:text-gray-600 mx-auto mb-3 opacity-50" />
            <p class="text-sm sm:text-base">No services found</p>
          </div>
          <div v-else class="space-y-3">
            <div
              v-for="(service, index) in services"
              :key="index"
              class="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-3 sm:p-4"
            >
              <div class="flex items-start gap-2 mb-2">
                <Layers class="w-5 h-5 text-blue-600 dark:text-blue-400 shrink-0 mt-0.5" />
                <div class="flex-1 min-w-0">
                  <h3 class="font-semibold text-gray-900 dark:text-white text-sm sm:text-base">Service</h3>
                  <p class="text-xs sm:text-sm text-gray-600 dark:text-gray-400 font-mono break-all">
                    {{ service.uuid }}
                  </p>
                </div>
              </div>
              <div v-if="service.characteristics && service.characteristics.length > 0" class="mt-3 space-y-2">
                <div
                  v-for="(characteristic, charIndex) in service.characteristics"
                  :key="charIndex"
                  class="ml-4 sm:ml-6 p-2 sm:p-3 bg-gray-50 dark:bg-gray-700 rounded border border-gray-200 dark:border-gray-600 hover:border-blue-400 dark:hover:border-blue-500 transition-colors"
                >
                  <div class="flex items-start justify-between gap-2 mb-2">
                    <div class="flex-1 min-w-0">
                      <p class="text-xs sm:text-sm font-medium text-gray-700 dark:text-gray-300">Characteristic</p>
                      <button
                        @click="selectCharacteristicAndNavigate(characteristic.uuid)"
                        :title="`Click to select in Communication tab`"
                        class="text-xs text-blue-600 dark:text-blue-400 font-mono break-all text-left hover:underline hover:text-blue-700 dark:hover:text-blue-300 transition-colors"
                      >
                        {{ characteristic.uuid }}
                      </button>
                    </div>
                  </div>
                  
                  <!-- Properties Display -->
                  <div class="flex flex-wrap gap-1 mt-2">
                    <!-- Read -->
                    <span v-if="getCharacteristicProperties(characteristic.properties).read" class="inline-flex items-center gap-1 text-xs px-2 py-1 bg-green-100 dark:bg-green-900 text-green-900 dark:text-green-100 rounded font-medium" title="Supports reading data">
                      <Eye class="w-3 h-3" /> Read
                    </span>
                    <span v-else class="inline-flex items-center gap-1 text-xs px-2 py-1 bg-gray-200 dark:bg-gray-600 text-gray-500 dark:text-gray-400 rounded opacity-50">
                      Read
                    </span>
                    
                    <!-- Write -->
                    <span v-if="getCharacteristicProperties(characteristic.properties).write" class="inline-flex items-center gap-1 text-xs px-2 py-1 bg-blue-100 dark:bg-blue-900 text-blue-900 dark:text-blue-100 rounded font-medium" title="Supports writing data">
                      <Edit3 class="w-3 h-3" /> Write
                    </span>
                    <span v-else class="inline-flex items-center gap-1 text-xs px-2 py-1 bg-gray-200 dark:bg-gray-600 text-gray-500 dark:text-gray-400 rounded opacity-50">
                      Write
                    </span>
                    
                    <!-- Write Without Response -->
                    <span v-if="getCharacteristicProperties(characteristic.properties).writeWithoutResponse" class="inline-flex items-center gap-1 text-xs px-2 py-1 bg-cyan-100 dark:bg-cyan-900 text-cyan-900 dark:text-cyan-100 rounded font-medium" title="Supports writing without response">
                      <Edit3 class="w-3 h-3" /> WriteNoResp
                    </span>
                    <span v-else class="inline-flex items-center gap-1 text-xs px-2 py-1 bg-gray-200 dark:bg-gray-600 text-gray-500 dark:text-gray-400 rounded opacity-50">
                      WriteNoResp
                    </span>
                    
                    <!-- Notify -->
                    <span v-if="getCharacteristicProperties(characteristic.properties).notify" class="inline-flex items-center gap-1 text-xs px-2 py-1 bg-purple-100 dark:bg-purple-900 text-purple-900 dark:text-purple-100 rounded font-medium" title="Supports notifications">
                      <Bell class="w-3 h-3" /> Notify
                    </span>
                    <span v-else class="inline-flex items-center gap-1 text-xs px-2 py-1 bg-gray-200 dark:bg-gray-600 text-gray-500 dark:text-gray-400 rounded opacity-50">
                      Notify
                    </span>
                    
                    <!-- Indicate -->
                    <span v-if="getCharacteristicProperties(characteristic.properties).indicate" class="inline-flex items-center gap-1 text-xs px-2 py-1 bg-orange-100 dark:bg-orange-900 text-orange-900 dark:text-orange-100 rounded font-medium" title="Supports indication">
                      <AlertCircle class="w-3 h-3" /> Indicate
                    </span>
                    <span v-else class="inline-flex items-center gap-1 text-xs px-2 py-1 bg-gray-200 dark:bg-gray-600 text-gray-500 dark:text-gray-400 rounded opacity-50">
                      Indicate
                    </span>
                  </div>
                  
                  <!-- Support Summary -->
                  <div class="mt-2 text-xs text-gray-600 dark:text-gray-400">
                    <span v-if="!characteristic.properties || (
                      !getCharacteristicProperties(characteristic.properties).read && 
                      !getCharacteristicProperties(characteristic.properties).write && 
                      !getCharacteristicProperties(characteristic.properties).writeWithoutResponse && 
                      !getCharacteristicProperties(characteristic.properties).notify && 
                      !getCharacteristicProperties(characteristic.properties).indicate
                    )" class="text-orange-600 dark:text-orange-400 font-medium">
                      ⚠️ No operations supported
                    </span>
                    <span v-else>
                      Supports:
                      <span v-if="getCharacteristicProperties(characteristic.properties).read" class="text-green-600 dark:text-green-400">Read</span>
                      <span v-if="getCharacteristicProperties(characteristic.properties).write" class="text-blue-600 dark:text-blue-400">{{getCharacteristicProperties(characteristic.properties).read ? ', ' : ''}}Write</span>
                      <span v-if="getCharacteristicProperties(characteristic.properties).writeWithoutResponse" class="text-blue-600 dark:text-blue-400">{{(getCharacteristicProperties(characteristic.properties).read || getCharacteristicProperties(characteristic.properties).write) ? ', ' : ''}}WriteNoResp</span>
                      <span v-if="getCharacteristicProperties(characteristic.properties).notify" class="text-purple-600 dark:text-purple-400">{{(getCharacteristicProperties(characteristic.properties).read || getCharacteristicProperties(characteristic.properties).write || getCharacteristicProperties(characteristic.properties).writeWithoutResponse) ? ', ' : ''}}Notify</span>
                      <span v-if="getCharacteristicProperties(characteristic.properties).indicate" class="text-orange-600 dark:text-orange-400">{{(getCharacteristicProperties(characteristic.properties).read || getCharacteristicProperties(characteristic.properties).write || getCharacteristicProperties(characteristic.properties).writeWithoutResponse || getCharacteristicProperties(characteristic.properties).notify) ? ', ' : ''}}Indicate</span>
                    </span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Communication Tab -->
        <div v-if="activeTab === 'Communication'" class="space-y-4">
          <div v-if="!connectedDevice" class="text-center py-8 text-gray-600 dark:text-gray-400">
            <Send class="w-12 h-12 sm:w-16 sm:h-16 text-gray-400 dark:text-gray-600 mx-auto mb-3 opacity-50" />
            <p class="text-sm sm:text-base">Connect to a device to communicate</p>
          </div>
          <div v-else class="space-y-4">
            <!-- Operation Feedback -->
            <div
              v-if="operationFeedbackVisible && operationFeedback"
              :class="[
                'p-4 rounded-lg border flex items-start gap-3 animate-pulse',
                operationFeedback.type === 'success'
                  ? 'bg-green-50 dark:bg-green-900/30 border-green-200 dark:border-green-800'
                  : operationFeedback.type === 'error'
                  ? 'bg-red-50 dark:bg-red-900/30 border-red-200 dark:border-red-800'
                  : 'bg-yellow-50 dark:bg-yellow-900/30 border-yellow-200 dark:border-yellow-800'
              ]"
            >
              <AlertCircle
                :class="[
                  'w-5 h-5 sm:w-6 sm:h-6 mt-0.5 shrink-0',
                  operationFeedback.type === 'success'
                    ? 'text-green-600 dark:text-green-400'
                    : operationFeedback.type === 'error'
                    ? 'text-red-600 dark:text-red-400'
                    : 'text-yellow-600 dark:text-yellow-400'
                ]"
              />
              <div class="flex-1">
                <p
                  :class="[
                    'text-sm',
                    operationFeedback.type === 'success'
                      ? 'text-green-700 dark:text-green-200'
                      : operationFeedback.type === 'error'
                      ? 'text-red-700 dark:text-red-200'
                      : 'text-yellow-700 dark:text-yellow-200'
                  ]"
                >
                  {{ operationFeedback.message }}
                </p>
              </div>
              <button
                @click="operationFeedbackVisible = false"
                class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 shrink-0 font-bold text-lg leading-none"
              >
                ×
              </button>
            </div>
            <!-- UUID Input with Dropdown -->
            <div class="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-4 sm:p-5">
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                <Code class="w-4 h-4 inline mr-2" />
                Characteristic UUID
              </label>
              <div class="space-y-2">
                <!-- Display current selection -->
                <div v-if="selectedCharacteristicUUID" class="p-2 bg-blue-50 dark:bg-blue-900/30 rounded border border-blue-200 dark:border-blue-700">
                  <p class="text-xs text-blue-600 dark:text-blue-400 mb-1">Current Selection:</p>
                  <p class="font-mono text-xs sm:text-sm text-blue-900 dark:text-blue-100 break-all">{{ selectedCharacteristicUUID }}</p>
                </div>
                
                <!-- Dropdown Selection -->
                <div class="relative group">
                  <button
                    class="w-full px-3 sm:px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-sm text-left font-mono focus:outline-none focus:ring-2 focus:ring-blue-600 dark:focus:ring-blue-400 flex items-center justify-between"
                  >
                    <span class="truncate">{{ selectedCharacteristicUUID || '-- Select a Characteristic --' }}</span>
                    <svg class="fill-current h-4 w-4 shrink-0 ml-2" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20">
                      <path d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z" />
                    </svg>
                  </button>
                  
                  <!-- Dropdown Menu -->
                  <div class="absolute left-0 right-0 top-full mt-1 bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-lg shadow-lg z-50 max-h-64 overflow-y-auto hidden group-hover:block">
                    <button
                      @click="selectedCharacteristicUUID = ''"
                      class="w-full text-left px-3 sm:px-4 py-2 text-sm text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-600 border-b border-gray-200 dark:border-gray-600"
                    >
                      -- Clear Selection --
                    </button>
                    <button
                      v-for="char in availableCharacteristics"
                      :key="char.uuid"
                      @click="selectedCharacteristicUUID = char.uuid"
                      :class="[
                        'w-full text-left px-3 sm:px-4 py-2 text-xs sm:text-sm font-mono border-b border-gray-100 dark:border-gray-600 last:border-0 transition-colors',
                        selectedCharacteristicUUID === char.uuid
                          ? 'bg-blue-100 dark:bg-blue-900 text-blue-900 dark:text-blue-100'
                          : 'text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-600'
                      ]"
                    >
                      <div class="flex items-center justify-between gap-2">
                        <span class="break-all">{{ char.uuid }}</span>
                        <span class="text-xs px-2 py-1 bg-gray-200 dark:bg-gray-600 text-gray-700 dark:text-gray-300 rounded shrink-0">
                          {{ char.serviceName.slice(-8) }}
                        </span>
                      </div>
                    </button>
                  </div>
                </div>
              </div>
              
              <!-- Manual input option -->
              <div class="mt-3">
                <p class="text-xs text-gray-600 dark:text-gray-400 mb-2">Or manually enter UUID:</p>
                <input
                  v-model="selectedCharacteristicUUID"
                  type="text"
                  placeholder="e.g., 51FF12BB-3ED8-46E5-B4F9-D64E2FEC021B"
                  class="w-full px-3 sm:px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-sm font-mono placeholder-gray-500 dark:placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-600 dark:focus:ring-blue-400"
                />
              </div>
            </div>

            <!-- Read Section -->
            <div class="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-4 sm:p-5">
              <div class="flex items-start justify-between mb-3">
                <h3 class="font-semibold text-gray-900 dark:text-white flex items-center gap-2">
                  <Eye class="w-5 h-5 text-green-600 dark:text-green-400" />
                  Read Data
                </h3>
                <span
                  v-if="currentCharacteristicProperties?.read"
                  class="inline-flex items-center gap-1 text-xs px-2 py-1 bg-green-100 dark:bg-green-900 text-green-900 dark:text-green-100 rounded"
                >
                  ✅ Supports Read
                </span>
                <span
                  v-else
                  class="inline-flex items-center gap-1 text-xs px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-400 rounded"
                >
                  ❌ Does Not Support Read
                </span>
              </div>
              <div class="flex gap-2 mb-3">
                <button
                  @click="readData"
                  :disabled="isLoading || !selectedCharacteristicUUID || !currentCharacteristicProperties?.read"
                  :title="!currentCharacteristicProperties?.read ? 'This Characteristic does not support read operation' : ''"
                  class="flex-1 px-4 py-2 rounded-lg bg-green-600 hover:bg-green-700 dark:bg-green-700 dark:hover:bg-green-600 text-white font-medium text-sm transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  Read
                </button>
                <button
                  @click="readDataString"
                  :disabled="isLoading || !selectedCharacteristicUUID || !currentCharacteristicProperties?.read"
                  :title="!currentCharacteristicProperties?.read ? 'This Characteristic does not support read operation' : ''"
                  class="flex-1 px-4 py-2 rounded-lg bg-green-600 hover:bg-green-700 dark:bg-green-700 dark:hover:bg-green-600 text-white font-medium text-sm transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  Read as String
                </button>
              </div>
              <div v-if="readDataValue" class="mt-2 p-3 bg-gray-100 dark:bg-gray-700 rounded border border-gray-300 dark:border-gray-600">
                <p class="text-xs sm:text-sm text-gray-600 dark:text-gray-400 mb-1">Data (Bytes):</p>
                <p class="font-mono text-xs sm:text-sm text-gray-900 dark:text-white break-all">{{ readDataValue }}</p>
              </div>
            </div>

            <!-- Write Section -->
            <div class="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-4 sm:p-5">
              <div class="flex items-start justify-between mb-3">
                <h3 class="font-semibold text-gray-900 dark:text-white flex items-center gap-2">
                  <Edit3 class="w-5 h-5 text-blue-600 dark:text-blue-400" />
                  Write Data
                </h3>
                <span
                  v-if="currentCharacteristicProperties?.write"
                  class="inline-flex items-center gap-1 text-xs px-2 py-1 bg-green-100 dark:bg-green-900 text-green-900 dark:text-green-100 rounded"
                >
                  ✅ Supports Write
                </span>
                <span
                  v-else
                  class="inline-flex items-center gap-1 text-xs px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-400 rounded"
                >
                  ❌ Does Not Support Write
                </span>
              </div>
              <div class="space-y-3">
                <textarea
                  v-model="writeDataString"
                  placeholder="Enter text to send..."
                  class="w-full px-3 sm:px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-sm placeholder-gray-500 dark:placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-600 dark:focus:ring-blue-400 resize-none h-24"
                />
                <div class="flex gap-2 flex-wrap sm:flex-nowrap">
                  <button
                    @click="sendString('withResponse')"
                    :disabled="isLoading || !selectedCharacteristicUUID || !writeDataString || !currentCharacteristicProperties?.write"
                    :title="!currentCharacteristicProperties?.write ? 'This Characteristic does not support write operation' : ''"
                    class="flex-1 px-4 py-2 rounded-lg bg-blue-600 hover:bg-blue-700 dark:bg-blue-700 dark:hover:bg-blue-600 text-white font-medium text-sm transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                  >
                    Send (With Response)
                  </button>
                  <button
                    @click="sendString('withoutResponse')"
                    :disabled="isLoading || !selectedCharacteristicUUID || !writeDataString || !currentCharacteristicProperties?.write"
                    :title="!currentCharacteristicProperties?.write ? 'This Characteristic does not support write operation' : ''"
                    class="flex-1 px-4 py-2 rounded-lg bg-blue-600 hover:bg-blue-700 dark:bg-blue-700 dark:hover:bg-blue-600 text-white font-medium text-sm transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                  >
                    Send (No Response)
                  </button>
                </div>
              </div>
            </div>

            <!-- Subscribe Section -->
            <div class="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-4 sm:p-5">
              <div class="flex items-start justify-between mb-3">
                <h3 class="font-semibold text-gray-900 dark:text-white flex items-center gap-2">
                  <Bell class="w-5 h-5 text-purple-600 dark:text-purple-400" />
                  Subscribe to Notifications
                </h3>
                <div class="flex gap-1">
                  <span
                    v-if="currentCharacteristicProperties?.notify"
                    class="inline-flex items-center gap-1 text-xs px-2 py-1 bg-green-100 dark:bg-green-900 text-green-900 dark:text-green-100 rounded"
                  >
                    ✅ Notify
                  </span>
                  <span
                    v-if="currentCharacteristicProperties?.indicate"
                    class="inline-flex items-center gap-1 text-xs px-2 py-1 bg-green-100 dark:bg-green-900 text-green-900 dark:text-green-100 rounded"
                  >
                    ✅ Indicate
                  </span>
                  <span
                    v-if="!currentCharacteristicProperties?.notify && !currentCharacteristicProperties?.indicate"
                    class="inline-flex items-center gap-1 text-xs px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-400 rounded"
                  >
                    ❌ Does Not Support Subscribe
                  </span>
                </div>
              </div>
              <div class="flex gap-2 mb-3 flex-wrap sm:flex-nowrap">
                <button
                  @click="subscribeToCharacteristic"
                  :disabled="isLoading || !selectedCharacteristicUUID || (!currentCharacteristicProperties?.notify && !currentCharacteristicProperties?.indicate)"
                  :title="!currentCharacteristicProperties?.notify && !currentCharacteristicProperties?.indicate ? 'This Characteristic does not support subscribe operation' : ''"
                  class="flex-1 px-4 py-2 rounded-lg bg-purple-600 hover:bg-purple-700 dark:bg-purple-700 dark:hover:bg-purple-600 text-white font-medium text-sm transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  Subscribe
                </button>
                <button
                  @click="unsubscribeFromCharacteristic"
                  :disabled="isLoading || !selectedCharacteristicUUID"
                  class="flex-1 px-4 py-2 rounded-lg bg-gray-600 hover:bg-gray-700 dark:bg-gray-700 dark:hover:bg-gray-600 text-white font-medium text-sm transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  Unsubscribe
                </button>
              </div>
              <div v-if="subscriptionData.length > 0" class="max-h-48 overflow-y-auto space-y-2">
                <div v-for="(data, index) in subscriptionData" :key="index" class="p-2 sm:p-3 bg-gray-100 dark:bg-gray-700 rounded text-xs sm:text-sm">
                  <p class="text-gray-600 dark:text-gray-400 mb-1">{{ new Date(data.timestamp).toLocaleTimeString() }}</p>
                  <p class="font-mono text-gray-900 dark:text-white break-all">{{ data.value }}</p>
                </div>
              </div>
              <div v-else class="text-center py-4 text-gray-600 dark:text-gray-400 text-sm">
                No notifications received yet
              </div>
            </div>
          </div>
        </div>

        <!-- Debug Tab -->
        <div v-if="activeTab === 'Debug'" class="space-y-4">
          <div class="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-4 sm:p-5">
            <h3 class="font-semibold text-gray-900 dark:text-white mb-3 flex items-center gap-2">
              <Terminal class="w-5 h-5 text-gray-600 dark:text-gray-400" />
              Connection Information
            </h3>
            <div class="space-y-2 font-mono text-xs sm:text-sm text-gray-600 dark:text-gray-400">
              <div class="flex justify-between gap-2">
                <span>Device Name:</span>
                <span class="text-gray-900 dark:text-white">{{ connectedDevice?.name || 'N/A' }}</span>
              </div>
              <div class="flex justify-between gap-2">
                <span>Device Address:</span>
                <span class="text-gray-900 dark:text-white break-all">{{ connectedDevice?.address || 'N/A' }}</span>
              </div>
              <div class="flex justify-between gap-2">
                <span>RSSI:</span>
                <span class="text-gray-900 dark:text-white">{{ connectedDevice?.rssi ?? 'N/A' }}</span>
              </div>
              <div class="flex justify-between gap-2">
                <span>Adapter State:</span>
                <span class="text-gray-900 dark:text-white">{{ adapterState }}</span>
              </div>
              <div class="flex justify-between gap-2">
                <span>Connection Phase:</span>
                <span :class="[
                  'font-semibold',
                  connectionPhase === 'idle' ? 'text-gray-500 dark:text-gray-400' :
                  connectionPhase === 'connecting' ? 'text-yellow-600 dark:text-yellow-400' :
                  connectionPhase === 'connected' ? 'text-blue-600 dark:text-blue-400' :
                  'text-green-600 dark:text-green-400'
                ]">
                  {{ connectionPhase }}
                </span>
              </div>
            </div>
          </div>

          <div class="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 p-4 sm:p-5">
            <div class="flex items-center justify-between mb-3">
              <h3 class="font-semibold text-gray-900 dark:text-white flex items-center gap-2">
                <FileText class="w-5 h-5 text-gray-600 dark:text-gray-400" />
                Log Output
              </h3>
              <div class="flex gap-2">
                <button
                  @click="copyDiagnosticsToClipboard"
                  class="px-3 py-1 text-xs rounded bg-blue-200 dark:bg-blue-700 text-blue-900 dark:text-blue-100 hover:bg-blue-300 dark:hover:bg-blue-600 transition-colors"
                  title="Copy diagnostics to clipboard"
                >
                  Copy
                </button>
                <button
                  @click="exportDiagnosticsAsJSON"
                  class="px-3 py-1 text-xs rounded bg-green-200 dark:bg-green-700 text-green-900 dark:text-green-100 hover:bg-green-300 dark:hover:bg-green-600 transition-colors"
                  title="Export diagnostics as JSON file"
                >
                  Export
                </button>
                <button
                  @click="clearLogs"
                  class="px-3 py-1 text-xs rounded bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-white hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors"
                >
                  Clear
                </button>
              </div>
            </div>
            <div class="max-h-60 overflow-y-auto bg-gray-900 dark:bg-black rounded p-3 font-mono text-xs text-green-400 space-y-1">
              <div v-for="(log, index) in logs" :key="index" class="whitespace-pre-wrap break-all">
                {{ log }}
              </div>
              <div v-if="logs.length === 0" class="text-gray-600">No logs yet</div>
            </div>
          </div>
        </div>
      </div>

      <!-- Connected Device Actions -->
      <div v-if="connectedDevice" class="fixed bottom-4 sm:bottom-6 right-4 sm:right-6 flex gap-2">
        <button
          @click="disconnectDevice"
          :disabled="isLoading"
          class="px-4 sm:px-6 py-2 sm:py-3 rounded-lg bg-red-600 hover:bg-red-700 dark:bg-red-700 dark:hover:bg-red-600 text-white font-medium text-sm sm:text-base shadow-lg transition-all hover:shadow-xl disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
        >
          <LogOut class="w-4 h-4 sm:w-5 sm:h-5" />
          <span class="hidden sm:inline">Disconnect</span>
        </button>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue'
import {
  Bluetooth,
  Wifi,
  Check,
  Activity,
  Link2,
  Link2Off,
  Signal,
  Loader,
  AlertCircle,
  Layers,
  Eye,
  Edit3,
  Bell,
  Send,
  Code,
  Terminal,
  FileText,
  LogOut,
} from 'lucide-vue-next'
import {
  BleDevice,
  BleService,
  startScan as bleStartScan,
  sendString as bleSendString,
  readString as bleReadString,
  unsubscribe as bleUnsubscribe,
  subscribeString as bleSubscribeString,
  stopScan as bleStopScan,
  connect as bleConnect,
  disconnect as bleDisconnect,
  getAdapterState as bleGetAdapterState,
  checkPermissions as bleCheckPermissions,
  read as bleRead,
  listServices as bleListServices,
} from '@mnlphlp/plugin-blec'

interface SubscriptionDataItem {
  timestamp: number
  value: string
}

// Type for parsed characteristic properties
interface CharacteristicProperties {
  read: boolean
  write: boolean
  writeWithoutResponse: boolean
  notify: boolean
  indicate: boolean
}

// Parse properties from properties number (BLE properties bitmask)
// 支持完整的 GATT 特征属性位定义
// enableLogging: 仅用于调试，生产环境应该关闭
const parseProperties = (propertiesNum: number, enableLogging: boolean = false): CharacteristicProperties => {
  // 使用完整的 GATT 特征属性定义
  // 参考 Android BluetoothGattCharacteristic 和标准 BLE GATT 规范
  const props: CharacteristicProperties = {
    // Bit 1: Read
    read: !!(propertiesNum & 0x02),
    // Bit 2: Write Without Response
    writeWithoutResponse: !!(propertiesNum & 0x04),
    // Bit 3: Write
    write: !!(propertiesNum & 0x08),
    // Bit 4: Notify
    notify: !!(propertiesNum & 0x10),
    // Bit 5: Indicate
    indicate: !!(propertiesNum & 0x20),
    // 注意：Bit 0 是 Broadcast, Bit 6 是 Signed Write, Bit 7 是 Extended Properties
  }
  
  // 仅在调试模式下记录日志
  if (enableLogging) {
    const binary = (propertiesNum >>> 0).toString(2).padStart(8, '0')
    addLog(`[PROPS_DEBUG] Raw value: ${propertiesNum} (0x${propertiesNum.toString(16).padStart(2, '0')}) Binary: ${binary}`)
    
    const supported = []
    if (props.read) supported.push('Read')
    if (props.write) supported.push('Write')
    if (props.writeWithoutResponse) supported.push('WriteNoResp')
    if (props.notify) supported.push('Notify')
    if (props.indicate) supported.push('Indicate')
    addLog(`        Parsed as: ${supported.length > 0 ? supported.join('+') : 'NONE'}`)
  }
  
  return props
}

// 🔧 新增：基于描述符推断特征属性（针对 @mnlphlp/plugin-blec 在 Android 上的 BUG）
// 库在 Android 上硬编码 properties 为 0，所以需要基于描述符和 UUID 推断
const inferPropertiesFromDescriptors = (descriptors: string[], uuid: string): CharacteristicProperties => {
  const props: CharacteristicProperties = {
    read: false,
    write: false,
    writeWithoutResponse: false,
    notify: false,
    indicate: false,
  }
  
  // UUID 标准映射
  const lowerUuid = uuid.toLowerCase()
  
  // 检查是否有 CCCD (Client Characteristic Configuration Descriptor: 0x2902)
  const hasCCCD = descriptors.some(d => d.toLowerCase().includes('2902'))
  // 检查是否有 SCCD (Server Characteristic Configuration Descriptor: 0x2903)
  const hasSCCD = descriptors.some(d => d.toLowerCase().includes('2903'))
  
  if (hasCCCD) {
    // CCCD 表示支持 Notify 或 Indicate，大多数情况是 Notify
    props.notify = true
  }
  
  if (hasSCCD) {
    // SCCD 表示支持 Indicate
    props.indicate = true
  }
  
  // 检查其他常见描述符
  const hasCharacteristicUserDescription = descriptors.some(d => d.toLowerCase().includes('2901'))
  
  // 基于 UUID 的标准 BLE 特征推断
  // Device Name 特征 (0x2a00) - 通常是可读的
  if (lowerUuid.includes('2a00')) {
    props.read = true
    props.write = true
    return props
  }
  
  // Model Number (0x2a24), Manufacturer Name (0x2a29), Serial Number (0x2a25) - 标准只读
  if (lowerUuid.includes('2a24') || lowerUuid.includes('2a29') || lowerUuid.includes('2a25')) {
    props.read = true
    return props
  }
  
  // 如果已经推断了 Notify 或 Indicate，返回
  if (props.notify || props.indicate) {
    return props
  }
  
  // 自定义 UUID 的启发式推断
  // 如果没有任何描述符，假设支持基本读写操作
  if (descriptors.length === 0) {
    // 对于完全没有描述符的特征，假设支持读、写、写无响应
    props.read = true
    props.write = true
    props.writeWithoutResponse = true
    return props
  }
  
  // 如果只有用户描述，通常是可读的
  if (descriptors.length === 1 && hasCharacteristicUserDescription) {
    props.read = true
    return props
  }
  
  // 其他情况：假设支持读操作
  props.read = true
  return props
}

// State
const devices = ref<Map<string, BleDevice>>(new Map())
const connectedDevice = ref<BleDevice | null>(null)
const isScanning = ref(false)
const isLoading = ref(false)
const isLoadingServices = ref(false)
const adapterState = ref<string>('Unknown')
const activeTab = ref<string>('Devices')
const services = ref<BleService[]>([])
const selectedCharacteristicUUID = ref<string>('')
const readDataValue = ref<string>('')
const writeDataString = ref<string>('')
const subscriptionData = ref<SubscriptionDataItem[]>([])
const logs = ref<string[]>([])
const connectionError = ref<string>('')
const connectionErrorVisible = ref<boolean>(false)
const searchQuery = ref<string>('')
const operationFeedback = ref<{ message: string; type: 'success' | 'error' | 'warning' } | null>(null)
const operationFeedbackVisible = ref<boolean>(false)
const connectionPhase = ref<'idle' | 'connecting' | 'connected' | 'loaded'>('idle')

const tabs = ['Devices', 'Services', 'Communication', 'Debug']

// Computed: Get all characteristic UUIDs from services
const availableCharacteristics = computed(() => {
  const characteristics: Array<{ uuid: string; serviceName: string }> = []
  for (const service of services.value) {
    if (service.characteristics && service.characteristics.length > 0) {
      for (const char of service.characteristics) {
        characteristics.push({
          uuid: char.uuid,
          serviceName: service.uuid,
        })
      }
    }
  }
  return characteristics
})

// Computed: Filter devices based on search query
const filteredDevices = computed(() => {
  if (!searchQuery.value.trim()) {
    return Array.from(devices.value.values())
  }
  
  const query = searchQuery.value.toLowerCase()
  return Array.from(devices.value.values()).filter((device) => {
    const name = (device.name || 'Unknown Device').toLowerCase()
    const address = device.address.toLowerCase()
    return name.includes(query) || address.includes(query)
  })
})

// Computed: Get current selected characteristic properties
const currentCharacteristicProperties = computed<CharacteristicProperties | null>(() => {
  if (!selectedCharacteristicUUID.value) return null
  
  for (const service of services.value) {
    if (service.characteristics) {
      for (const char of service.characteristics) {
        if (char.uuid === selectedCharacteristicUUID.value) {
          // char.properties is a number (bitmask), parse it to get boolean properties
          return char.properties !== undefined ? parseProperties(char.properties) : { read: false, write: false, writeWithoutResponse: false, notify: false, indicate: false }
        }
      }
    }
  }
  return null
})

// Utility Functions
// Helper function to get parsed properties for a characteristic (used in template)
// Helper function to get parsed properties for a characteristic (used in template)
// 处理两种输入：原始数字 或 已经解析的对象
const getCharacteristicProperties = (input: number | CharacteristicProperties | undefined): CharacteristicProperties => {
  if (input === undefined || input === null) {
    return { read: false, write: false, writeWithoutResponse: false, notify: false, indicate: false }
  }
  
  // 如果是对象，直接返回
  if (typeof input === 'object' && !Array.isArray(input)) {
    return input as CharacteristicProperties
  }
  
  // 如果是数字，解析它
  if (typeof input === 'number') {
    return parseProperties(input)
  }
  
  return { read: false, write: false, writeWithoutResponse: false, notify: false, indicate: false }
}

const addLog = (message: string) => {
  const timestamp = new Date().toLocaleTimeString()
  const logEntry = `[${timestamp}] ${message}`
  
  // 如果消息包含换行符，按行分割并逐行添加（性能优化）
  if (message.includes('\n')) {
    const lines = message.split('\n')
    lines.forEach(line => {
      if (line.trim()) {
        logs.value.unshift(`[${timestamp}] ${line}`)
      }
    })
  } else {
    logs.value.unshift(logEntry)
  }
  
  // 限制日志数量，避免内存溢出
  if (logs.value.length > 150) {
    logs.value.splice(100) // 超过 150 条时，保留最新的 100 条
  }
}

// Show operation feedback
const showFeedback = (message: string, type: 'success' | 'error' | 'warning' = 'success', duration = 3000) => {
  operationFeedback.value = { message, type }
  operationFeedbackVisible.value = true
  setTimeout(() => {
    operationFeedbackVisible.value = false
  }, duration)
}

// Create a timeout promise
const createTimeoutPromise = (ms: number): Promise<never> => {
  return new Promise((_, reject) => {
    setTimeout(() => {
      reject(new Error(`Operation timeout (${ms}ms)`))
    }, ms)
  })
}

// Race a promise with a timeout
const withTimeout = async <T>(promise: Promise<T>, timeoutMs: number): Promise<T> => {
  return Promise.race([promise, createTimeoutPromise(timeoutMs)])
}

// Convert adapter state to standard format
const normalizeAdapterState = (state: any): string => {
  if (typeof state === 'boolean') {
    return state ? 'PoweredOn' : 'PoweredOff'
  }
  const stateStr = String(state).toLowerCase()
  if (stateStr === 'on' || stateStr === 'true' || stateStr === 'poweredon') {
    return 'PoweredOn'
  }
  if (stateStr === 'off' || stateStr === 'false' || stateStr === 'poweredoff') {
    return 'PoweredOff'
  }
  return String(state) || 'Unknown'
}

const clearLogs = () => {
  logs.value = []
}

// 导出诊断信息为JSON
const exportDiagnosticsAsJSON = () => {
  const diagnostics = {
    timestamp: new Date().toISOString(),
    platform: 'Android',
    adapterState: adapterState.value,
    connectedDevice: connectedDevice.value ? {
      name: connectedDevice.value.name,
      address: connectedDevice.value.address,
      rssi: connectedDevice.value.rssi,
    } : null,
    services: services.value.map(service => ({
      uuid: service.uuid,
      characteristicsCount: service.characteristics?.length || 0,
      characteristics: service.characteristics?.map(char => ({
        uuid: char.uuid,
        propertiesRaw: char.properties,
        propertiesHex: `0x${(char.properties || 0).toString(16).padStart(2, '0')}`,
        propertiesBinary: ((char.properties || 0) >>> 0).toString(2).padStart(8, '0'),
        properties: parseProperties(char.properties || 0),
      })) || [],
    })),
    logs: logs.value,
  }
  
  const jsonStr = JSON.stringify(diagnostics, null, 2)
  const blob = new Blob([jsonStr], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `ble-diagnostics-${Date.now()}.json`
  a.click()
  URL.revokeObjectURL(url)
  
  addLog('📥 Diagnostics exported to JSON file')
  showFeedback('✅ Diagnostics exported successfully', 'success')
}

// 复制诊断信息到剪贴板
const copyDiagnosticsToClipboard = async () => {
  const diagnostics = {
    timestamp: new Date().toISOString(),
    platform: 'Android',
    adapterState: adapterState.value,
    connectedDevice: connectedDevice.value ? {
      name: connectedDevice.value.name,
      address: connectedDevice.value.address,
      rssi: connectedDevice.value.rssi,
    } : null,
    services: services.value.map(service => ({
      uuid: service.uuid,
      characteristicsCount: service.characteristics?.length || 0,
      characteristics: service.characteristics?.map(char => ({
        uuid: char.uuid,
        propertiesRaw: char.properties,
        propertiesHex: `0x${(char.properties || 0).toString(16).padStart(2, '0')}`,
        propertiesBinary: ((char.properties || 0) >>> 0).toString(2).padStart(8, '0'),
        properties: parseProperties(char.properties || 0),
      })) || [],
    })),
  }
  
  const jsonStr = JSON.stringify(diagnostics, null, 2)
  try {
    await navigator.clipboard.writeText(jsonStr)
    addLog('📋 Diagnostics copied to clipboard')
    showFeedback('✅ Diagnostics copied to clipboard', 'success')
  } catch (error) {
    addLog(`❌ Failed to copy: ${String(error)}`)
    showFeedback('❌ Failed to copy to clipboard', 'error')
  }
}

// BLE Functions
const checkPermissions = async () => {
  try {
    isLoading.value = true
    addLog('Checking permissions...')
    const result = await bleCheckPermissions()
    addLog(`Permissions checked: ${JSON.stringify(result)}`)
    return result
  } catch (error) {
    addLog(`Error checking permissions: ${String(error)}`)
    return false
  } finally {
    isLoading.value = false
  }
}

// 🔐 新增：确保权限已获取（内部函数，用于连接前检查）
const ensureBluetoothPermissions = async (): Promise<boolean> => {
  try {
    addLog('[PERMISSION] Checking Bluetooth permissions...')
    const hasPermission = await bleCheckPermissions()
    
    if (hasPermission) {
      addLog('[PERMISSION] ✅ Bluetooth permissions already granted')
      return true
    } else {
      // 权限未授予 - 这在 Android 12+ 上很关键！
      addLog('[PERMISSION] ❌ Bluetooth permissions NOT granted')
      addLog('[PERMISSION] ⚠️ Please ensure BLUETOOTH_CONNECT and BLUETOOTH_SCAN permissions are granted in Settings')
      showFeedback(
        'Bluetooth permissions required. Please grant them in Settings.',
        'warning',
        5000
      )
      return false
    }
  } catch (error) {
    addLog(`[PERMISSION] Error checking permissions: ${String(error)}`)
    return false
  }
}

const toggleScan = async () => {
  if (isScanning.value) {
    await stopScan()
  } else {
    await startScan()
  }
}

const startScan = async () => {
  try {
    isLoading.value = true
    searchQuery.value = '' // Reset search when starting scan
    addLog('Starting scan...')
    isScanning.value = true
    devices.value.clear()

    // Use startScan with callback and timeout (10 seconds)
    await bleStartScan(
      (foundDevices: BleDevice[]) => {
        devices.value.clear()
        for (const device of foundDevices) {
          if (device && device.address) {
            devices.value.set(device.address, device)
            addLog(`Device found: ${device.name || device.address}`)
          }
        }
      },
      10000, // Timeout: 10 seconds
      false  // Don't require location permission
    )

    addLog('Scan completed')
    isScanning.value = false
  } catch (error) {
    addLog(`Error starting scan: ${String(error)}`)
    isScanning.value = false
  } finally {
    isLoading.value = false
  }
}

const stopScan = async () => {
  try {
    isLoading.value = true
    addLog('Stopping scan...')
    await bleStopScan()
    isScanning.value = false
  } catch (error) {
    addLog(`Error stopping scan: ${String(error)}`)
  } finally {
    isLoading.value = false
  }
}

const connectToDevice = async (device: BleDevice) => {
  try {
    isLoading.value = true
    connectionError.value = ''
    connectionErrorVisible.value = false
    connectionPhase.value = 'connecting'  // ← 标记为"正在连接"
    addLog(`[CONNECT] 开始连接到 ${device.name || device.address}...`)
    addLog(`[CONNECT] 设置 connectionPhase = 'connecting'`)
    
    // 🔐 权限检查（新增！）
    addLog(`[CONNECT] 检查权限...`)
    const hasPermissions = await ensureBluetoothPermissions()
    if (!hasPermissions) {
      addLog(`[CONNECT] ❌ 权限不足，连接失败`)
      connectionPhase.value = 'idle'
      connectionError.value = 'Bluetooth permissions required'
      connectionErrorVisible.value = true
      return
    }
    addLog(`[CONNECT] ✅ 权限检查通过，继续连接...`)

    // Connect with 20 second timeout
    await withTimeout(
      bleConnect(device.address, async () => {
        // ← 关键：只在真正连接后才允许这个回调清空状态
        // 防止在 'connecting' 或 'connected' 阶段误触发
        addLog(`[DISCONNECT_CALLBACK] 触发！当前 connectionPhase = '${connectionPhase.value}'`)
        
        if (connectionPhase.value === 'loaded' || (connectionPhase.value === 'idle' && connectedDevice.value === null)) {
          addLog(`[DISCONNECT_CALLBACK] 状态检查通过，执行断开逻辑`)
          addLog('Device disconnected')
          connectedDevice.value = null
          services.value = []
          selectedCharacteristicUUID.value = ''
          readDataValue.value = ''
          writeDataString.value = ''
          subscriptionData.value = []
          connectionPhase.value = 'idle'
        } else {
          addLog(`[DISCONNECT_CALLBACK] ⚠️ 状态检查失败，忽略此回调（防止误触发）`)
        }
      }),
      20000 // 20 second timeout
    )

    // ← 标记为"已连接"
    connectionPhase.value = 'connected'
    addLog(`[CONNECT] 设置 connectionPhase = 'connected'`)
    connectedDevice.value = device
    addLog(`[CONNECT] 已连接到 ${device.name || device.address}`)
    activeTab.value = 'Services'

    // Load services
    addLog(`[CONNECT] 开始加载服务...`)
    await loadServices()
    
    // ← 标记为"已加载"
    connectionPhase.value = 'loaded'
    addLog(`[CONNECT] 设置 connectionPhase = 'loaded'`)
    addLog(`[CONNECT] ✅ 连接流程完成！`)
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error)
    connectionError.value = `Connection failed: ${errorMessage}`
    connectionErrorVisible.value = true
    addLog(`[CONNECT] ❌ 错误: ${errorMessage}`)
    connectedDevice.value = null
    connectionPhase.value = 'idle'
    addLog(`[CONNECT] 重置为 connectionPhase = 'idle'`)
    
    // Auto-hide error after 5 seconds
    setTimeout(() => {
      connectionErrorVisible.value = false
    }, 5000)
  } finally {
    isLoading.value = false
  }
}

const disconnectDevice = async () => {
  try {
    isLoading.value = true
    if (connectedDevice.value) {
      addLog(`Disconnecting from ${connectedDevice.value.name || connectedDevice.value.address}...`)
      await bleDisconnect()
      connectedDevice.value = null
      connectionPhase.value = 'idle'  // ← 标记为空闲
      services.value = []
      selectedCharacteristicUUID.value = ''
      readDataValue.value = ''
      writeDataString.value = ''
      subscriptionData.value = []
    }
  } catch (error) {
    addLog(`Error disconnecting: ${String(error)}`)
  } finally {
    isLoading.value = false
  }
}

const loadServices = async () => {
  try {
    isLoadingServices.value = true
    if (!connectedDevice.value) return

    addLog('Loading services...')
    const servicesList = await withTimeout(
      bleListServices(connectedDevice.value.address),
      10000 // 10 second timeout for loading services
    )
    
    // 确保 servicesList 是数组类型
    let servicesArray = Array.isArray(servicesList) ? servicesList : [servicesList]
    
    // 类型安全过滤
    servicesArray = (servicesArray as any[]).filter((s: any) => typeof s === 'object' && s !== null && s.uuid)
    
    // 🔍 检查是否所有特征属性都是 0（可能是库的问题）
    const allPropsZero = (servicesArray as any[]).every((service: any) => 
      !service.characteristics || service.characteristics.length === 0 || 
      service.characteristics.every((char: any) => char.properties === 0)
    )
    
    const hasCharacteristics = (servicesArray as any[]).some((s: any) => s.characteristics?.length)
    
    if (allPropsZero && hasCharacteristics) {
      const logFn = addLog  // 将 addLog 保存到局部变量
      logFn('🔧 [AUTO_FIX_PROPS] 检测到 @mnlphlp/plugin-blec Android 的已知问题：properties 硬编码为 0')
      logFn('🔧 [AUTO_FIX_PROPS] 原因：库在 Peripheral.kt 中 hardcode properties=0')
      logFn('🔧 [AUTO_FIX_PROPS] 使用基于描述符的推断机制来恢复特征属性...')
      
      // 基于描述符推断属性
      const inferredLogs: string[] = []
      ;(servicesArray as any[]).forEach((service: any) => {
        if (service.characteristics?.length) {
          service.characteristics.forEach((char: any) => {
            const descriptors = Array.isArray(char.descriptors) ? char.descriptors : []
            const inferredProps = inferPropertiesFromDescriptors(descriptors, char.uuid)
            
            // 将推断的属性转换为 bitmask 格式
            let propsBitmask = 0
            if (inferredProps.read) propsBitmask |= 0x02
            if (inferredProps.writeWithoutResponse) propsBitmask |= 0x04
            if (inferredProps.write) propsBitmask |= 0x08
            if (inferredProps.notify) propsBitmask |= 0x10
            if (inferredProps.indicate) propsBitmask |= 0x20
            
            char.properties = propsBitmask
            inferredLogs.push(`  ✓ ${char.uuid}: inferred as 0x${propsBitmask.toString(16).padStart(2, '0')}`)
          })
        }
      })
      
      // 输出所有推断日志
      inferredLogs.forEach(log => logFn(log))
      
      services.value = servicesArray as BleService[]
      logFn('✅ [AUTO_FIX_PROPS] 属性推断完成！')
    } else {
      services.value = servicesArray as BleService[]
    }
    
    addLog(`Loaded ${services.value.length} services`)
    
    // 🔍 调试第1步：检查原始库返回的数据
    addLog(`[STEP1] Raw API response inspection:`)
    addLog(`  servicesList type: ${typeof servicesList}`)
    addLog(`  servicesList is array: ${Array.isArray(servicesList)}`)
    addLog(`  servicesList length: ${services.value.length}`)
    
    // 🔍 调试第2步：逐个检查特征属性
    let propsDebugLog = `[STEP2] Characteristic properties (raw from API):\n`
    services.value.forEach((service: any, sIdx: number) => {
      propsDebugLog += `\n  Service[${sIdx}] ${service.uuid}:\n`
      if (service.characteristics?.length) {
        service.characteristics.forEach((char: any, cIdx: number) => {
          const props = char.properties
          propsDebugLog += `    Char[${cIdx}] ${char.uuid}:\n`
          propsDebugLog += `      .properties = ${props}\n`
          propsDebugLog += `      type = ${typeof props}\n`
          propsDebugLog += `      raw object = ${JSON.stringify(char)}\n`
        })
      } else {
        propsDebugLog += `    ⚠️ No characteristics\n`
      }
    })
    addLog(propsDebugLog)
    
    // 详细日志：列出所有特征及其属性（优化：减少 DOM 更新）
    // 使用 setTimeout 让日志异步处理，不阻塞 UI 更新
    setTimeout(() => {
      let detailLog = ''
      
      servicesArray.forEach((service: any, serviceIndex: number) => {
        detailLog += `\n  Service [${serviceIndex}]: ${service.uuid}`
        if (service.characteristics && service.characteristics.length > 0) {
          detailLog += ` (${service.characteristics.length} chars)`
          service.characteristics.forEach((char: any, charIndex: number) => {
            const props = char.properties !== undefined ? char.properties : 0
            const binary = (props >>> 0).toString(2).padStart(8, '0')
            detailLog += `\n    Char [${charIndex}]: ${char.uuid}`
            detailLog += `\n      Props: 0x${props.toString(16).padStart(2, '0')} (${binary})`
            
            // 解析属性（不记录日志，避免卡顿）
            const parsed = parseProperties(props, false)
            const supportedOps = []
            if (parsed.read) supportedOps.push('Read')
            if (parsed.write) supportedOps.push('Write')
            if (parsed.writeWithoutResponse) supportedOps.push('WriteNoResp')
            if (parsed.notify) supportedOps.push('Notify')
            if (parsed.indicate) supportedOps.push('Indicate')
            
            if (supportedOps.length === 0) {
              detailLog += ` ⚠️ NO OPS`
            } else {
              detailLog += ` ✅ ${supportedOps.join(', ')}`
            }
          })
        } else {
          detailLog += ` (⚠️ No chars)`
        }
      })
      
      // 一次性记录所有详情
      addLog(detailLog)
    }, 0)
  } catch (error) {
    addLog(`Error loading services: ${String(error)}`)
  } finally {
    isLoadingServices.value = false
  }
}

const readData = async () => {
  try {
    // Check if read is supported
    if (!currentCharacteristicProperties.value?.read) {
      showFeedback('This Characteristic does not support read operation', 'warning')
      addLog('Read operation not supported for this characteristic')
      return
    }

    isLoading.value = true
    addLog(`Reading from ${selectedCharacteristicUUID.value}...`)
    const data = await withTimeout(bleRead(selectedCharacteristicUUID.value), 10000)
    readDataValue.value = data.toString()
    addLog(`Read: ${readDataValue.value}`)
    showFeedback(`✅ Read successful (${data.toString().length} bytes)`, 'success')
  } catch (error) {
    const errorMsg = error instanceof Error ? error.message : String(error)
    addLog(`Error reading: ${errorMsg}`)
    showFeedback(`❌ Read failed: ${errorMsg}`, 'error')
  } finally {
    isLoading.value = false
  }
}

const readDataString = async () => {
  try {
    // Check if read is supported
    if (!currentCharacteristicProperties.value?.read) {
      showFeedback('This Characteristic does not support read operation', 'warning')
      addLog('Read operation not supported for this characteristic')
      return
    }

    isLoading.value = true
    addLog(`Reading string from ${selectedCharacteristicUUID.value}...`)
    const data = await withTimeout(bleReadString(selectedCharacteristicUUID.value), 10000)
    readDataValue.value = data
    addLog(`Read string: ${readDataValue.value}`)
    showFeedback(`✅ Read successful: "${data}"`, 'success')
  } catch (error) {
    const errorMsg = error instanceof Error ? error.message : String(error)
    addLog(`Error reading string: ${errorMsg}`)
    showFeedback(`❌ Read failed: ${errorMsg}`, 'error')
  } finally {
    isLoading.value = false
  }
}

const sendString = async (writeType: 'withResponse' | 'withoutResponse') => {
  try {
    // Check if write is supported
    if (!currentCharacteristicProperties.value?.write) {
      showFeedback('This Characteristic does not support write operation', 'warning')
      addLog('Write operation not supported for this characteristic')
      return
    }

    if (!writeDataString.value.trim()) {
      showFeedback('Please enter data to send', 'warning')
      return
    }

    isLoading.value = true
    const msgType = writeType === 'withResponse' ? '(with response)' : '(without response)'
    addLog(`Sending "${writeDataString.value}" with ${writeType}...`)
    await withTimeout(
      bleSendString(selectedCharacteristicUUID.value, writeDataString.value, writeType),
      10000
    )
    addLog(`Sent successfully`)
    const sentData = writeDataString.value
    writeDataString.value = ''
    showFeedback(`✅ Send successful ${msgType}: "${sentData}"`, 'success')
  } catch (error) {
    const errorMsg = error instanceof Error ? error.message : String(error)
    addLog(`Error sending: ${errorMsg}`)
    showFeedback(`❌ Send failed: ${errorMsg}`, 'error')
  } finally {
    isLoading.value = false
  }
}

const subscribeToCharacteristic = async () => {
  try {
    // Check if notify or indicate is supported
    const supportsNotify = currentCharacteristicProperties.value?.notify
    const supportsIndicate = currentCharacteristicProperties.value?.indicate
    
    if (!supportsNotify && !supportsIndicate) {
      showFeedback('This Characteristic does not support subscribe operation (no Notify/Indicate)', 'warning')
      addLog('Subscribe operation not supported for this characteristic')
      return
    }

    isLoading.value = true
    addLog(`Subscribing to ${selectedCharacteristicUUID.value}...`)
    subscriptionData.value = []

    const unsubscribe = await withTimeout(
      bleSubscribeString(selectedCharacteristicUUID.value, (data: string) => {
        subscriptionData.value.unshift({
          timestamp: Date.now(),
          value: data,
        })
        if (subscriptionData.value.length > 50) {
          subscriptionData.value.pop()
        }
        addLog(`Received: ${data}`)
      }),
      10000
    )

    // Store unsubscribe function
    ;(globalThis as any).currentSubscription = unsubscribe
    addLog('Subscribed successfully')
    const notifyType = supportsNotify ? 'Notify' : supportsIndicate ? 'Indicate' : ''
    showFeedback(`✅ Subscribe successful (${notifyType})`, 'success')
  } catch (error) {
    const errorMsg = error instanceof Error ? error.message : String(error)
    addLog(`Error subscribing: ${errorMsg}`)
    showFeedback(`❌ Subscribe failed: ${errorMsg}`, 'error')
  } finally {
    isLoading.value = false
  }
}

const unsubscribeFromCharacteristic = async () => {
  try {
    isLoading.value = true
    addLog(`Unsubscribing from ${selectedCharacteristicUUID.value}...`)

    if ((globalThis as any).currentSubscription) {
      (globalThis as any).currentSubscription()
    }

    await bleUnsubscribe(selectedCharacteristicUUID.value)
    addLog('Unsubscribed successfully')
    showFeedback('✅ Unsubscribe successful', 'success')
  } catch (error) {
    const errorMsg = error instanceof Error ? error.message : String(error)
    addLog(`Error unsubscribing: ${errorMsg}`)
    showFeedback(`❌ Unsubscribe failed: ${errorMsg}`, 'error')
  } finally {
    isLoading.value = false
  }
}

// Select characteristic and navigate to Communication tab
const selectCharacteristicAndNavigate = (uuid: string) => {
  selectedCharacteristicUUID.value = uuid
  activeTab.value = 'Communication'
  // Scroll to top of Communication tab
  setTimeout(() => {
    const element = document.querySelector('[data-tab="communication"]')
    element?.scrollIntoView({ behavior: 'smooth', block: 'start' })
  }, 100)
}

// Lifecycle
onMounted(async () => {
  try {
    addLog('App mounted, checking adapter state...')
    const state = await bleGetAdapterState()
    adapterState.value = normalizeAdapterState(state)
    addLog(`Adapter state: ${adapterState.value}`)

    // Only check adapter state periodically, don't use getConnectionUpdates
    // getConnectionUpdates is for device connection events, not adapter state
    // Instead, we'll periodically poll the adapter state
    setInterval(async () => {
      try {
        const newState = await bleGetAdapterState()
        const normalizedState = normalizeAdapterState(newState)
        if (normalizedState !== adapterState.value) {
          adapterState.value = normalizedState
          addLog(`Adapter state changed: ${adapterState.value}`)
        }
      } catch (error: unknown) {
        // Silently ignore errors in polling to avoid spam in logs
        const errorMsg = error instanceof Error ? error.message : String(error)
        // Only log if error is significant
        if (errorMsg && !errorMsg.includes('timeout')) {
          addLog(`Adapter state check error: ${errorMsg}`)
        }
      }
    }, 2000) // Check every 2 seconds
  } catch (error) {
    addLog(`Error on mount: ${String(error)}`)
  }
}
)

onUnmounted(() => {
  // Cleanup
  if ((globalThis as any).scanUnsubscribe) {
    (globalThis as any).scanUnsubscribe()
  }
  if ((globalThis as any).currentSubscription) {
    (globalThis as any).currentSubscription()
  }
  if ((globalThis as any).adapterStateUnsubscribe) {
    (globalThis as any).adapterStateUnsubscribe()
  }
})
</script>

<style scoped>
::-webkit-scrollbar {
  width: 6px;
  height: 6px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: rgb(209, 213, 219);
  border-radius: 3px;
}

::-webkit-scrollbar-thumb:hover {
  background: rgb(156, 163, 175);
}

@media (prefers-color-scheme: dark) {
  ::-webkit-scrollbar-thumb {
    background: rgb(75, 85, 99);
  }

  ::-webkit-scrollbar-thumb:hover {
    background: rgb(107, 114, 128);
  }
}
</style>