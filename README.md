# NSE - Nintendo Switch Emulator

### Requirements

- Apply Hypervisor for Macos: https://developer.apple.com/documentation/hypervisor

Build virtualization solutions on top of a lightweight hypervisor, without third-party kernel extensions.

Run this command to see if your MacOS supports Hypervisor `sysctl -a | grep kern.hv`

### Ryujinx Architecture

#### `public boool loadNsp(string path)`

Method used to load the nsp file

```cs
public bool LoadNsp(string path)
  {
      FileStream file = new(path, FileMode.Open, FileAccess.Read);
      PartitionFileSystem partitionFileSystem = new(file.AsStorage());

      (bool success, ProcessResult processResult) = partitionFileSystem.TryLoad(_device, path, out string errorMessage);

      if (processResult.ProcessId == 0)
      {
          // This is not a normal NSP, it's actually a ExeFS as a NSP
          processResult = partitionFileSystem.Load(_device, new BlitStruct<ApplicationControlProperty>(1), partitionFileSystem.GetNpdm(), true);
      }

      if (processResult.ProcessId != 0 && _processesByPid.TryAdd(processResult.ProcessId, processResult))
      {
          if (processResult.Start(_device))
          {
              _latestPid = processResult.ProcessId;

              return true;
          }
      }

      if (!success)
      {
          Logger.Error?.Print(LogClass.Loader, errorMessage, nameof(PartitionFileSystemExtensions.TryLoad));
      }

      return false;
  }
```

### Milestone

- [ ] Firmware configuration

https://theprodkeys.com/switch-firmwares-download/

Yuzu / Ryujinx Emulators Switch emulators sometimes require Switch firmware. Through this switch firmware we can run many games on the switch emulator without any problem. So let’s know about the latest switch firmwares topic.

Playing Nintendo Switch games on Ryujinx emulators requires firmware with a product key. Switch games cannot be played if this emulator does not have firmware. That’s why everyone will need it.

```
@chungquantin: We can reuse the firmware loader of Yuzu or Ryujinx. Because I use MacOS, I prefer Ryujinx
```

- [ ] Scaffold the specification of Switch CPU
- [ ] Define list of required instruction set
- [ ] Prod keys handler
- [ ] Game file loader (NSP and XCI)
- [ ] Run `Legends of Zelda: Breath of the Wild`
- [ ] Port to mobile
