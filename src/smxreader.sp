
/*
SPDX-License-Identifier: WTFPL
Copyright 2022 rtldg <rtldg@protonmail.com>

DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
    0. You just DO WHAT THE FUCK YOU WANT TO.
*/

#define TARGET_SECTION ".src.zip"
#define TARGET_SIZE 15

bool ReadSection(File file, int stringtab)
{
	int nameoffs, dataoffs, size;
	file.ReadInt32(nameoffs);
	file.ReadInt32(dataoffs);
	file.ReadInt32(size);

	int pos = file.Position;
	file.Seek(stringtab + nameoffs, SEEK_SET);

	char name[64];
	file.ReadString(name, sizeof(name));
	file.Seek(pos, SEEK_SET);

	return size > TARGET_SIZE && StrEqual(name, TARGET_SECTION);
}

bool CheckMyShit(File file)
{
	int magic, version, compression, disksize, imagesize, sections, stringtab, dataoffs;
	file.ReadInt32(magic);
	file.ReadUint16(version);
	file.ReadUint8(compression);
	file.ReadInt32(disksize);
	file.ReadInt32(imagesize);
	file.ReadUint8(sections);
	file.ReadInt32(stringtab);
	file.ReadInt32(dataoffs);
	// file position is now at the start of the sections

	for (int i = 0; i < sections; i++)
		if (ReadSection(file, stringtab))
			return true;

	return false;
}

public APLRes AskPluginLoad2(Handle myself, bool late, char[] error, int err_max)
{
	char relative_filename[PLATFORM_MAX_PATH], full_path[PLATFORM_MAX_PATH];
	GetPluginFilename(INVALID_HANDLE, relative_filename, sizeof(relative_filename));
	BuildPath(Path_SM, full_path, sizeof(full_path), "plugins/%s", relative_filename);
	File file = OpenFile(full_path, "rb");

	bool ok = CheckMyShit(file);
	PrintToServer("smxreader ok = %d", ok);
	delete file;

	return APLRes_Success;
}
